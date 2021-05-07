pub fn pass1_1028_525a(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_52e8(param_1: *mut Struct763) -> *mut Struct763 {
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

pub fn pass1_1028_5338(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_53c6(param_1: *mut Struct763) -> *mut Struct763 {
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
    let pp_var1: fn();
    let mut u_var2: u16;
    let mut u_var3: u32;
    let mut i_var4: i32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var2 = (param_1 >> 0x10);
    if ((param_1 + 0x12) != 6) {
        return;
    }
    u_var3 = pass1_1028_b4f2(param_1);
    if ((u_var3 + 0x200) != 0x8000002) {
        if ((param_1 + 0x1c) == 0x8000002) {
            i_var4 = 6;
            // goto code_r0x1028548e;
        }
        pp_var1 = (param_1 + 100);
        i_var4 = (**pp_var1)();
        if (i_var4 == 0) {
            return;
        }
        i_var4 = pass1_1028_c0f0(param_1, u_var2, 1, 0);
        if (i_var4 == 0) {
            i_var4 = 6;
            // goto code_r0x1028548e;
        }
        pass1_1028_c952(param_1);
        pass1_1028_c00a(param_1, u_var2, 1, 0);
    }
    i_var4 = 5;
    // code_r0x1028548e:
    pass1_1028_bdac(param_1, i_var4);
    return;
}

pub fn pass1_1028_5496(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_5524(param_1: *mut Struct727) -> *mut Struct763 {
    pass1_1028_0068(param_1);
    param_1 = 0x55c8;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_5546(
    param_1: *mut Struct728,
    param_2: u16,
    param_3: u16,
    param_3_00: u32,
) -> *mut Struct728 {
    pass1_1028_00cc(param_1, CONCAT22(param_3, param_2), param_3_00);
    *CONCAT22(param_2, param_1) = 0x55c8;
    param_1.field_0x2 = &PTR_LOOP_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_5570(param_1: u32) {
    let u_var1: u8;
    let extraout_AH: u8;
    let mut extraout_dx: u16;
    let mut u_var2: u16;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let mut u_var5: i32;
    let mut u_var6: u16;

    u_var5 = (param_1 >> 0x10);
    pass1_1028_0550(param_1);
    if ((param_1 + 0x12) == 5) {
        u_var4 = 0;
        u_var6 = 4;
        u_var3 = 1;
        u_var2 = extraout_dx;
        u_var1 = pass1_1028_b58e((param_1 & 0xffff | u_var5 << 0x10));
        pass1_1030_7c50(
            CONCAT22(u_var2, CONCAT11(extraout_AH, u_var1)),
            CONCAT22(u_var4, u_var3),
            u_var6,
        );
    }
    return;
}

pub fn pass1_1028_55a2(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1028_0138(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_5630(param_1: *mut Struct763) -> *mut Struct763 {
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

pub fn pass1_1028_5686(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_5718(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_57a6(param_1: *mut Struct763) -> *mut Struct763 {
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

pub fn pass1_1028_57f6(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_5884(param_1: *mut Struct763) -> *mut Struct763 {
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

pub fn pass1_1028_58d8(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_5966(param_1: *mut Struct763) -> *mut Struct763 {
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

pub fn pass1_1028_59ba(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_5a48(param_1: *mut Struct763) -> *mut Struct763 {
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

pub fn pass1_1028_5a94(param_1: *mut Struct44, param_2: *mut u32) {
    let pp_var1: fn();
    let u_var2: u8;
    let mut in_AX: i32;
    let pu_var3: *mut u8;
    let mut u_var4: u32;
    let extraout_var: u32;
    let mut extraout_dx: i32;
    let mut extraout_dx_00: i32;
    let mut u_var5: i32;
    let mut unaff_ss: u16;
    let paVar6: *mut Struct967;
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
        pp_var1 = (*param_2 + 0x10);
        (**pp_var1)();
    }
    _local_6 = CONCAT22(extraout_dx, in_AX);
    if ((extraout_dx | in_AX) == 0) {
        return;
    }
    local_8 = 1;
    pass1_1030_bcae(local_a, unaff_ss);
    local_e = 0;
    while (true) {
        if (_local_6 <= local_e) {
            return;
        }
        u_var4 = _local_6;
        pass1_1030_1d58(param_2);
        paVar6 = (u_var4 & 0xffff | extraout_dx_00 << 0x10);
        u_var5 = extraout_dx_00;
        u_var2 = pass1_1028_b58e(param_1);
        pu_var3 = local_a;
        pass1_1030_bd74(
            pu_var3,
            unaff_ss,
            (CONCAT31(extraout_var, u_var2) & 0xffff | u_var5 << 0x10),
            paVar6,
        );
        if (pu_var3 < 5) {
            break;
        }
        local_e = local_e + 1;
    }
    pass1_1030_73a8((u_var4 & 0xffff | extraout_dx_00 << 0x10));
    return;
}

pub fn pass1_1028_5b42(param_1: *mut u32) {
    let pp_var1: fn();
    let mut u_var2: u16;
    let mut u_var3: u32;
    let mut i_var4: i32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var2 = (param_1 >> 0x10);
    if ((param_1 + 0x12) != 6) {
        return;
    }
    u_var3 = pass1_1028_b4f2(param_1);
    if ((u_var3 + 0x200) != 0x8000002) {
        if ((param_1 + 0x1c) == 0x8000002) {
            i_var4 = 6;
            // goto code_r0x10285bbe;
        }
        unsafe { pp_var1 = (*param_1 + 100) };
        i_var4 = (**pp_var1)();
        if (i_var4 == 0) {
            return;
        }
        i_var4 = pass1_1028_c0f0(param_1, u_var2, 2, 0);
        if (i_var4 == 0) {
            i_var4 = 6;
            // goto code_r0x10285bbe;
        }
        pass1_1028_c952(param_1);
        pass1_1028_c00a(param_1, u_var2, 2, 0);
    }
    i_var4 = 5;
    // code_r0x10285bbe:
    pass1_1028_bdac(param_1, i_var4);
    return;
}

pub fn pass1_1028_5bc6(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_5c54(param_1: *mut Struct763) -> *mut Struct763 {
    pass1_1028_b354(param_1);
    param_1.field_0x0 = (s_static_1050_5d8b + 3);
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_5c76(param_1: u16, param_2: u32, param_3: u32) {
    let mut u_var1: u16;

    u_var1 = param_2;
    pass1_1028_b39e(CONCAT22(u_var1, param_1), (param_2 >> 0x10), param_3);
    CONCAT22(u_var1, param_1) = (s_static_1050_5d8b + 3);
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_5ca0(param_1: u32) {
    let u_var1: u8;
    let extraout_var: u32;
    let mut in_dx: i32;
    let mut unaff_ss: u16;
    let mut u_var2: u32;
    let mut local_12e: u16;
    let mut local_12c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var1 = pass1_1028_b58e(param_1);
    _local_6 = CONCAT31(extraout_var, u_var1) & 0xffff | in_dx << 0x10;
    local_a = (CONCAT31(extraout_var, u_var1) + 0x2e);
    u_var2 = pass1_1028_bb24(param_1);
    pass1_1028_87f0(
        CONCAT22(unaff_ss, &local_12e),
        0,
        0,
        0x65,
        (_local_6 + 0xc),
        (_local_6 >> 0x10),
        (local_a + 4),
        u_var2,
    );
    u_var1 = &local_12e;
    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_12e));
    return u_var1;
}

pub fn pass1_1028_5d0e(param_1: *mut Struct44) {
    let u_var1: u8;
    let extraout_var: u32;
    let mut in_dx: i32;
    let mut unaff_ss: u16;
    let mut local_11c: u16;
    let mut local_11a: u16;
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var1 = pass1_1028_b58e(param_1);
    _local_6 = CONCAT31(extraout_var, u_var1) & 0xffff | in_dx << 0x10;
    local_a = (CONCAT31(extraout_var, u_var1) + 0x2e);
    local_e = (local_a + 4);
    pass1_1028_68de(CONCAT22(unaff_ss, &local_11c), 1, local_e);
    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_11c));
    return;
}

pub fn pass1_1028_5d68(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_5df6(param_1: *mut Struct763) -> *mut Struct763 {
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

pub fn pass1_1028_5e4a(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_5ed8(param_1: *mut Struct763) -> *mut Struct763 {
    let mut u_var1: u16;

    pass1_1028_b354(param_1);
    u_var1 = (param_1 >> 0x10);
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

pub fn pass1_1028_5f30(param_1: *mut Struct44) {
    let pi_var1: *mut i32;
    let u_var2: u8;
    let mut u_var3: u16;
    let extraout_var: u32;
    let pu_var4: *mut u8;
    let mut extraout_dx: u16;
    let mut u_var5: u16;
    let mut extraout_dx_00: i32;
    let mut i_var6: i32;
    let mut u_var7: u16;
    let mut i_var8: i32;
    let mut local_6: u16;
    let mut local_4: u16;

    i_var6 = param_1;
    u_var7 = (param_1 >> 0x10);
    pass1_1028_be9e(i_var6, u_var7);
    if ((i_var6 + 0x12) == 5) {
        (i_var6 + 0x20) = 100;
        u_var5 = extraout_dx;
        u_var2 = pass1_1028_b58e(param_1);
        pu_var4 = *(CONCAT31(extraout_var, u_var2) + 0x2e);
        i_var8 = 0x61;
        pass1_1038_3fb0(pu_var4);
        u_var3 = pass1_1030_25b2(pu_var4 & 0xffff | extraout_dx_00 << 0x10, i_var8);
        if (u_var3 != 0) {
            pi_var1 = (i_var6 + 0x20);
            unsafe { *pi_var1 = *pi_var1 + 100 };
        }
    }
    return;
}

pub fn pass1_1028_3e06(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1028_388e(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_3e94(param_1: *mut Struct763) -> *mut Struct763 {
    let local_struct_1: *mut Struct763;
    let mut u_var1: i32;

    pass1_1028_b354(param_1);
    u_var1 = (param_1 >> 0x10);
    local_struct_1 = param_1;
    local_struct_1.field_0x20 = 0;
    param_1.field_0x0 = 0x4004;
    local_struct_1.field_0x2 = &PTR_LOOP_1050_1028;
    pass1_1028_3fa2((param_1 & 0xffff | u_var1 << 0x10));
    return param_1;
}

pub fn pass1_1028_3ec8(
    param_1: *mut Struct763,
    param_2: u16,
    param_3: u16,
    param_3_00: u32,
) -> *mut Struct763 {
    pass1_1028_b39e(param_1, param_2, CONCAT22(param_3_00, param_3));
    (param_1 + 0x20) = 0;
    param_1.field_0x0 = 0x4004;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    pass1_1028_3fa2((param_1 & 0xffff | param_1._2_2_ << 0x10));
    return param_1;
}

pub fn pass1_1028_3f04(param_1: *mut Struct44) {
    let mut u_var1: u32;
    let u_var2: u8;
    let mut u_var3: i32;
    let extraout_var: u32;
    let mut u_var5: u16;
    let in_edx: u32;
    let mut i_var6: i32;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut u_var4: u32;

    u_var8 = 0x1f;
    u_var2 = pass1_1028_b58e(param_1);
    u_var4 = CONCAT31(extraout_var, u_var2);
    u_var7 = u_var4;
    u_var5 = in_edx;
    u_var1 = u_var4 & 0xffff | in_edx << 0x10;
    pass1_1030_7c28(u_var4 & 0xffff | in_edx << 0x10, u_var8);
    _local_a = CONCAT22(u_var5, u_var7);
    pass1_1030_7d1c(u_var1, 0, 0x1f0000);
    u_var7 = (param_1 >> 0x10);
    i_var6 = param_1;
    if ((i_var6 + 0xc) != 0x22) {
        if ((i_var6 + 0xc) == 0x23) {
            u_var3 = 10;
        } else {
            u_var3 = 5;
        }
        _local_e = u_var3;
        u_var4 = _local_a + (i_var6 + 0x20);
        (i_var6 + 0x20) = u_var4 % u_var3;
        _local_a = u_var4 + u_var4 / _local_e;
    }
    pass1_1030_7ddc(u_var1, _local_a, 0x21);
    return;
}

pub fn pass1_1028_3fa2(in_struct_1: *mut Struct763) {
    let mut u_var1: i32;
    let mut i_var2: i32;
    let local_bx_4: *mut Struct763;
    let mut u_var3: u16;

    u_var3 = (in_struct_1 >> 0x10);
    local_bx_4 = in_struct_1;
    if (&local_bx_4.field_0xc != 0x22) {
        if (&local_bx_4.field_0xc == 0x23) {
            u_var1 = 10;
        } else {
            u_var1 = 5;
        }
        i_var2 = pass1_fn_1008_612e(0, u_var1 >> 1);
        &local_bx_4.field_0x20 = i_var2;
        (&local_bx_4.field_0x20 + 2) = i_var2 >> 0xf;
    }
    return;
}

pub fn pass1_1028_3fde(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_406c(in_struct_1: *mut Struct763) -> *mut Struct763 {
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

pub fn pass1_1028_40b8(param_1: *mut Struct44, param_2: u32) {
    let pp_var1: fn();
    let u_var2: u8;
    let pu_var3: *mut u32;
    let paVar4: *mut Struct493;
    let extraout_var: u32;
    let extraout_var_00: u32;
    let mut in_dx: u16;
    let mut extraout_dx: u16;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut u_var6: u32;
    let mut u_var7: u32;
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

    u_var2 = pass1_1028_b58e(param_1);
    local_6 = CONCAT31(extraout_var, u_var2);
    local_14 = (local_6 + 0xc);
    local_8 = (local_6 + 0x10);
    local_1a = &local_c;
    local_22 = CONCAT22(local_22._2_2_, &local_14);
    local_10 = local_8 + 1;
    u_var7 = CONCAT22(unaff_ss, local_2c);
    local_e = local_10;
    local_c = local_14;
    u_var6 = pass1_1028_bb24(param_1);
    pu_var3 = &local_14;
    pass1_1030_64ce(
        _PTR_LOOP_1050_5740,
        pu_var3,
        unaff_ss,
        u_var6,
        (u_var6 >> 0x10),
        u_var7,
    );
    unsafe { local_22 = *pu_var3 };
    u_var5 = (pu_var3 + 2);
    local_36._3_1_ = (local_22 >> 0x18);
    local_18 = local_22;
    if (local_36._3_1_ != '\0') {
        paVar4 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_22, u_var5);
        local_36 = CONCAT22(u_var5, paVar4);
        u_var2 = pass1_1030_6fa0(CONCAT22(u_var5, paVar4));
        if (CONCAT31(extraout_var_00, u_var2) == 100) {
            _local_26 = pass1_1030_73a8(local_36);
            pp_var1 = (*_local_26 + 0x58);
            (**pp_var1)(0x1030, _local_26, (_local_26 >> 0x10), param_2);
        }
    }
    pass1_1028_b514(param_1);
    return;
}

pub fn pass1_1028_4194(param_1: u32) {
    let mut u_var1: u16;
    let mut u_var2: u32;
    let ppVar3: *mut pass1_struct_1;
    let mut in_stack_0000ffea: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1028_be9e(param_1);
    u_var2 = pass1_1028_b4f2(param_1);
    u_var1 = (u_var2 >> 0x10);
    if (((u_var2 + 0x200) != 0x8000002) && ((param_1 + 0x12) == 5)) {
        ppVar3 = process_struct_1010_20ba(
            ctx._g_Struct372_1050_0ed0,
            CONCAT22(in_stack_0000ffea, 0x2b),
        );
        pass1_1010_043a(ppVar3, (u_var2 + 4), 0xe);
    }
    return;
}

pub fn pass1_1028_42c6(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_4354(param_1: *mut Struct763) -> *mut Struct763 {
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
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
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
        ppVar1 =
            process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000fffa, 8));
        pass1_1010_988c(ppVar1, (param_1 + 0xc));
    }
    return;
}

pub fn pass1_1028_43f6(param_1: *mut Struct44, param_2: i32) {
    let u_var1: u8;
    let extraout_AH: u8;
    let mut u_var2: u16;
    let ppVar3: *mut pass1_struct_1;
    let mut u_var4: u32;
    let mut u_var5: u16;

    u_var5 = 0x83;
    ppVar3 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, 0x830009);
    u_var4 = pass1_1010_65d0(ppVar3, u_var5);
    u_var2 = (u_var4 >> 0x10);
    if (0 < u_var4) {
        u_var1 = pass1_1028_b58e(param_1);
        if (param_2 == 0) {
            u_var5 = 0;
        } else {
            u_var5 = 1000;
        }
        pass1_1030_7d1c(
            CONCAT22(u_var2, CONCAT11(extraout_AH, u_var1)),
            u_var5,
            0x230000,
        );
    }
    return;
}

pub fn pass1_1028_4444(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_44d2(param_1: *mut Struct763) -> *mut Struct763 {
    let mut u_var1: u16;

    pass1_1028_b354(param_1);
    u_var1 = (param_1 >> 0x10);
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

pub fn pass1_1028_4530(param_1: *mut Struct44) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let local_struct_1: *mut Struct44;
    let mut u_var4: u16;

    u_var4 = (param_1 >> 0x10);
    local_struct_1 = param_1;
    param_1.ptr_a_lo = 0x4836;
    local_struct_1.ptr_a_hi = &PTR_LOOP_1050_1028;
    pu_var1 = local_struct_1.field_0x20;
    u_var2 = &local_struct_1.field_0x22;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            ppc_var3 = *pu_var1;
            (**ppc_var3)();
        }
    }
    pass1_1028_b418(param_1);
    return;
}

pub fn pass1_1028_456e(param_1: *mut Struct781, param_2: *mut u8) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let local_bx_18: *mut Struct781;
    let mut u_var3: u16;
    let fn_ptr_1: fn();

    pass1_1028_b46e(param_1, param_2);
    u_var3 = (param_1 >> 0x10);
    local_bx_18 = param_1;
    pu_var1 = local_bx_18.field_0x20;
    u_var2 = &local_bx_18.field_0x22;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            fn_ptr_1 = *pu_var1;
            (**fn_ptr_1)()
        };
    }
    &local_bx_18.field_0x20 = 0;
    return;
}

pub fn pass1_1028_45b0(param_1: u32) {
    let u_var1: u8;
    let extraout_AH: u8;
    let mut extraout_dx: u16;
    let mut u_var2: u16;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let mut u_var5: i32;
    let mut u_var6: u16;

    u_var5 = (param_1 >> 0x10);
    pass1_1028_be9e(param_1, u_var5);
    if ((param_1 + 0x12) == 5) {
        u_var4 = 0;
        u_var6 = 4;
        u_var3 = 2;
        u_var2 = extraout_dx;
        u_var1 = pass1_1028_b58e((param_1 & 0xffff | u_var5 << 0x10));
        pass1_1030_7c50(
            CONCAT22(u_var2, CONCAT11(extraout_AH, u_var1)),
            CONCAT22(u_var4, u_var3),
            u_var6,
        );
    }
    return;
}

pub fn pass1_1028_45e2(param_1: u32) {
    let mut u_var1: u32;

    u_var1 = pass1_1028_478a(param_1);
    return CONCAT22(-(1000 < u_var1) - (u_var1 >> 0x10), 1000 - u_var1);
}

pub fn pass1_1028_45fe(param_1: *mut Struct44) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let u_var3: u8;
    let pu_var4: *mut u32;
    let pa_var5: *mut Struct199;
    let local_AX_178: *mut Struct779;
    let extraout_var: u32;
    let mut in_dx: i32;
    let extraout_dx: *mut Struct199;
    let mut u_var7: i32;
    let mut extraout_dx_00: i32;
    let mut i_var8: i32;
    let local_bx_265: *mut Struct780;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut unaff_ss: u16;
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

    u_var3 = pass1_1028_b58e(param_1);
    _local_6 = CONCAT31(extraout_var, u_var3) & 0xffff | in_dx << 0x10;
    local_a = (CONCAT31(extraout_var, u_var3) + 0x22);
    u_var9 = (param_1 >> 0x10);
    i_var8 = param_1;
    pu_var4 = (i_var8 + 0x20);
    pa_var5 = (i_var8 + 0x22);
    _local_1e = CONCAT22(pa_var5, pu_var4);
    local_22 = pu_var4;
    local_20 = pa_var5;
    if ((pa_var5 | pu_var4) != 0) {
        unsafe {
            ppc_var2 = *pu_var4;
            ppc_var2();
        }
        pa_var5 = extraout_dx;
    }
    process_struct_1000_179c(0xc, pa_var5);
    u_var7 = pa_var5 | pu_var4;
    local_22 = pu_var4;
    local_20 = pa_var5;
    if (u_var7 == 0) {
        pa_var5 = 0x0;
        u_var7 = 0;
    } else {
        pa_var5 = process_struct_1008_574a(CONCAT22(pa_var5, pu_var4));
    }
    (i_var8 + 0x20) = pa_var5;
    (i_var8 + 0x22) = u_var7;
    if (local_a != 0) {
        _local_e = *(local_a + 4);
        local_12 = 0;
        while (local_12 < _local_e) {
            pass1_1020_bb16(
                local_a,
                CONCAT22(unaff_ss, &local_28),
                CONCAT22(unaff_ss, &local_1a),
                local_12,
            );
            if ((local_28 != 0) && (local_1a != 0)) {
                puVar6 = _PTR_LOOP_1050_68a2;
                alloc_mem_1000_07fc(_PTR_LOOP_1050_68a2);
                u_var7 = puVar6;
                _local_1e = (puVar6 & 0xffff | extraout_dx_00 << 0x10);
                if ((extraout_dx_00 | u_var7) == 0) {
                    local_2c = 0;
                } else {
                    *_local_1e = ctx.s_1_1050_389a;
                    (u_var7 + 2) = &ctx.PTR_LOOP_1050_1008;
                    (u_var7 + 4) = 0;
                    (u_var7 + 6) = 0;
                    (u_var7 + 8) = 0;
                    (u_var7 + 10) = 0;
                    (u_var7 + 0xc) = 0;
                    *_local_1e = 0x56ce;
                    (u_var7 + 2) = 0x1018;
                    local_2c = _local_1e;
                }
                u_var10 = (local_2c >> 0x10);
                local_bx_265 = local_2c;
                local_bx_265.field_0x4 = local_1a;
                &local_bx_265.field_0xa = local_28;
                (&local_bx_265.field_0xa + 2) = local_28;
                u_var1 = (i_var8 + 0x20);
                ppc_var2 = ((i_var8 + 0x20) + 8);
                ppc_var2(0, u_var1, (u_var1 >> 0x10), local_bx_265, u_var10);
            }
            local_12 = local_12 + 1;
        }
    }
    return;
}

pub fn pass1_1028_4768(param_1: u32) {
    let mut u_var1: u32;

    u_var1 = pass1_1028_478a(param_1);
    if (((u_var1 >> 0x10) == 0) && (u_var1 < 1000)) {
        return 0;
    }
    return 1;
}

pub fn pass1_1028_478a(param_1: *mut Struct44) {
    let mut i_var1: i32;
    let u_var2: u8;
    let extraout_var: u32;
    let mut in_dx: i32;
    let mut unaff_ss: u16;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var2 = pass1_1028_b58e(param_1);
    i_var1 = CONCAT31(extraout_var, u_var2);
    _local_6 = CONCAT31(extraout_var, u_var2) & 0xffff | in_dx << 0x10;
    local_a = (i_var1 + 0x22);
    local_e = 0;
    if (((i_var1 + 0x24) | local_a) == 0) {
        return;
    }
    local_10 = (local_a + 4);
    local_12 = 0;
    while (local_12 < local_10) {
        pass1_1020_bb16(
            local_a,
            CONCAT22(unaff_ss, &local_1e),
            CONCAT22(unaff_ss, &local_1a),
            local_12,
        );
        if (0 < local_1a) {
            local_e = local_e + local_1e;
        }
        local_12 = local_12 + 1;
    }
    return;
}

pub fn pass1_1028_4810(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1028_4530(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_489e(param_1: *mut Struct763) -> *mut Struct763 {
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

pub fn pass1_1028_491c(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_49aa(param_1: *mut Struct763) -> *mut Struct763 {
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

pub fn pass1_1028_254c(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
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

pub fn pass1_1028_2626(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_26b4(param_1: *mut Struct763) -> *mut Struct763 {
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
    let mut u_var1: i32;
    let mut local_DXAX_27: u32;

    pass1_1028_be2a(param_1);
    u_var1 = (param_1 >> 0x10);
    if ((param_1 + 0x12) == 5) {
        local_DXAX_27 = pass1_1028_b4f2((param_1 & 0xffff | u_var1 << 0x10));
        (local_DXAX_27 + 0x204) = 1;
    }
    return;
}

pub fn pass1_1028_272e(param_1: u32) {
    let mut u_var1: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1028_be9e(param_1);
    u_var1 = pass1_1028_b4f2(param_1);
    if ((param_1 + 0x12) == 5) {
        (u_var1 + 0x204) = 1;
    }
    return;
}

pub fn pass1_1028_2762(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_27f0(param_1: *mut Struct763) -> *mut Struct763 {
    pass1_1028_b354(param_1);
    param_1.field_0x0 = (s_fem123_wav_1050_2a89 + 9);
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_2812(
    param_1: *mut Struct759,
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
    let mut u_var1: u16;
    let mut unaff_ss: u16;
    let mut u_var2: u16;
    let mut u_var3: u16;
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
        CONCAT22(unaff_ss, &local_8),
        CONCAT22(unaff_ss, local_e),
        CONCAT22(unaff_ss, &local_c),
        CONCAT22(unaff_ss, &local_a),
    );
    _local_8 = _local_8 & 0xffff | (local_c - 1) << 0x10;
    u_var2 = param_1;
    u_var3 = (param_1 >> 0x10);
    u_var1 = pass1_1028_c5a6(
        u_var2,
        u_var3,
        0x7b,
        CONCAT22(unaff_ss, &local_8),
        param_4_00,
    );
    if (u_var1 == 0) {
        u_var1 = switch_fn_1028_297c(param_1, CONCAT22(unaff_ss, &local_8), param_4_00);
        if (u_var1 == 0) {
            _local_8 = _local_8 & 0xffff | (local_c + 1) << 0x10;
            u_var1 = pass1_1028_c5a6(
                u_var2,
                u_var3,
                0x7b,
                CONCAT22(unaff_ss, &local_8),
                param_4_00,
            );
            if (u_var1 == 0) {
                u_var1 = switch_fn_1028_297c(param_1, CONCAT22(unaff_ss, &local_8), param_4_00);
                if (u_var1 == 0) {
                    local_8 = local_a - 1;
                    local_6 = local_c;
                    u_var1 = pass1_1028_c5a6(
                        u_var2,
                        u_var3,
                        0x7c,
                        CONCAT22(unaff_ss, &local_8),
                        param_4_00,
                    );
                    if (u_var1 == 0) {
                        u_var1 =
                            switch_fn_1028_297c(param_1, CONCAT22(unaff_ss, &local_8), param_4_00);
                        if (u_var1 == 0) {
                            _local_8 = CONCAT22(local_6, local_a + 1);
                            u_var1 = pass1_1028_c5a6(
                                u_var2,
                                u_var3,
                                0x7c,
                                CONCAT22(unaff_ss, &local_8),
                                param_4_00,
                            );
                            if (u_var1 == 0) {
                                u_var1 = switch_fn_1028_297c(
                                    param_1,
                                    CONCAT22(unaff_ss, &local_8),
                                    param_4_00,
                                );
                                if (u_var1 == 0) {
                                    return u_var1;
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
    let mut i_var2: i32;
    let paVar3: *mut Struct493;
    let mut u_var4: i32;
    let mut u_var5: i32;
    let lVar6: u32;
    let mut u_var7: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_6: u16;

    i_var2 = pass1_1028_c7b6(param_1, param_2, param_3);
    if (i_var2 == 0) {
        lVar6 = pass1_1030_627e(_PTR_LOOP_1050_5740, param_2, param_3);
        u_var4 = (lVar6 >> 0x10);
        u_var5 = u_var4 | lVar6;
        if (lVar6 != 0) {
            paVar3 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, lVar6, u_var4);
            u_var7 = pass1_1030_73a8(CONCAT22(u_var5, paVar3));
            u_var4 = (u_var7 + 0xc);
            if (0x4a < u_var4) {
                todo!();
                //                 switch (u_var4)
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
            if ((u_var4 < 0x48) && (u_var4 != 0x41)) {
                if (u_var4 < 0x42) {
                    cVar1 = u_var4;
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
    param_1: *mut Struct760,
    param_2: u16,
    param_3: u16,
    param_3_00: u32,
) -> u32 {
    pass1_1030_dcc2(param_1, param_2, param_3, param_3_00);
    *CONCAT22(param_2, param_1) = (s_add5_wav_1050_2b73 + 1);
    param_1.field_0x2 = &PTR_LOOP_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_2b4e(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1030_dcf4(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_2bdc(param_1: *mut Struct742) -> *mut Struct742 {
    pass1_1028_0954(param_1);
    param_1 = 0x341c;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_2bfe(param_1: *mut Struct743, param_2: u16, param_3: u16, param_3_00: u32) {
    pass1_1028_0982(param_1, CONCAT22(param_3, param_2), param_3_00);
    CONCAT22(param_2, param_1) = 0x341c;
    param_1.field_0x2 = &PTR_LOOP_1050_1028;
    return;
}

pub fn pass1_1028_2c28(param_1: u32, param_2: u32, param_3: u32, param_4: u32) {
    let mut i_var1: i32;
    let mut unaff_ss: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let mut uStack4: u16;

    i_var1 = pass1_1028_09d4(param_1, param_2, param_3, param_4);
    if (i_var1 != 0) {
        local_8 = param_2;
        uStack4 = (param_2 + 4);
        pass1_1008_3eb4(
            CONCAT22(unaff_ss, &local_8),
            CONCAT22(unaff_ss, &local_e),
            CONCAT22(unaff_ss, &local_c),
            CONCAT22(unaff_ss, &local_a),
        );
        pass1_1008_3e76(
            CONCAT22(unaff_ss, &local_8),
            local_e,
            local_c - 1,
            local_a - 1,
        );
        i_var1 = pass1_1028_c7b6(param_1, &local_8, unaff_ss, param_4);
        if (i_var1 != 0) {
            pass1_1008_3e76(CONCAT22(unaff_ss, &local_8), local_e, local_c - 1, local_a);
            i_var1 = pass1_1028_c7b6(param_1, &local_8, unaff_ss, param_4);
            if (i_var1 != 0) {
                pass1_1008_3e76(
                    CONCAT22(unaff_ss, &local_8),
                    local_e,
                    local_c - 1,
                    local_a + 1,
                );
                i_var1 = pass1_1028_c7b6(param_1, &local_8, unaff_ss, param_4);
                if (i_var1 != 0) {
                    pass1_1008_3e76(CONCAT22(unaff_ss, &local_8), local_e, local_c, local_a - 1);
                    i_var1 = pass1_1028_c7b6(param_1, &local_8, unaff_ss, param_4);
                    if (i_var1 != 0) {
                        pass1_1008_3e76(CONCAT22(unaff_ss, &local_8), local_e, local_c, local_a);
                        i_var1 = pass1_1028_c7b6(param_1, &local_8, unaff_ss, param_4);
                        if (i_var1 != 0) {
                            pass1_1008_3e76(
                                CONCAT22(unaff_ss, &local_8),
                                local_e,
                                local_c,
                                local_a + 1,
                            );
                            i_var1 = pass1_1028_c7b6(param_1, &local_8, unaff_ss, param_4);
                            if (i_var1 != 0) {
                                pass1_1008_3e76(
                                    CONCAT22(unaff_ss, &local_8),
                                    local_e,
                                    local_c + 1,
                                    local_a - 1,
                                );
                                i_var1 = pass1_1028_c7b6(param_1, &local_8, unaff_ss, param_4);
                                if (i_var1 != 0) {
                                    pass1_1008_3e76(
                                        CONCAT22(unaff_ss, &local_8),
                                        local_e,
                                        local_c + 1,
                                        local_a,
                                    );
                                    i_var1 = pass1_1028_c7b6(param_1, &local_8, unaff_ss, param_4);
                                    if (i_var1 != 0) {
                                        pass1_1008_3e76(
                                            CONCAT22(unaff_ss, &local_8),
                                            local_e,
                                            local_c + 1,
                                            local_a + 1,
                                        );
                                        i_var1 =
                                            pass1_1028_c7b6(param_1, &local_8, unaff_ss, param_4);
                                        if (i_var1 != 0) {
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
    let mut u_var2: u16;
    let mut u_var3: u16;
    let u_var4: u8;
    let u_var5: u8;
    let u_var6: u8;
    let u_var7: u8;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut in_stack_0000ffee: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    pass1_1028_09b8(param_1, param_2);
    if ((param_2 >> 0x10) == 0) {
        u_var9 = 0;
        u_var10 = 8;
        u_var6 = 1;
        u_var7 = 0;
        u_var8 = 0;
        u_var3 = 0;
        u_var4 = 0;
        u_var5 = 0;
        u_var2 = 0;
        ppVar1 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, 0x37);
        post_win_msg_1008_a0e4(
            ppVar1,
            CONCAT22(u_var3, u_var2),
            CONCAT11(u_var5, u_var4),
            CONCAT11(u_var7, u_var6),
            CONCAT22(u_var9, u_var8),
            u_var10,
        );
        u_var3 = 0x400;
        u_var10 = 3;
        u_var2 = 1;
        ppVar1 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, 0x1002b);
        pass1_1010_043a(ppVar1, CONCAT22(u_var3, u_var2), u_var10);
        pass1_1010_043a(ppVar1, 0x4000001, 4);
        ppVar1 = process_struct_1010_20ba(
            ctx._g_Struct372_1050_0ed0,
            CONCAT22(in_stack_0000ffee, 0x2f),
        );
        pass1_1010_ec84(ppVar1);
        ppVar1 =
            process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000ffee, 8));
        pass1_1010_9766(ppVar1);
    }
    return;
}

pub fn pass1_1028_2f18(param_1: *mut Struct44) {
    let mut i_var1: i32;
    let u_var2: u8;
    let pu_var3: *mut u32;
    let extraout_var: u32;
    let mut u_var4: i32;
    let mut extraout_dx: u16;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
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
    u_var4 = (_local_6 >> 0x10);
    u_var2 = pass1_1028_b58e(param_1);
    i_var1 = CONCAT31(extraout_var, u_var2);
    _local_a = CONCAT31(extraout_var, u_var2) & 0xffff | u_var4 << 0x10;
    local_e = (i_var1 + 0x2e);
    local_12 = (local_e + 4);
    local_18 = (i_var1 + 0xc);
    uStack20 = (i_var1 + 0x10);
    pass1_1008_3eb4(
        CONCAT22(unaff_ss, &local_18),
        CONCAT22(unaff_ss, &local_1e),
        CONCAT22(unaff_ss, &local_1c),
        CONCAT22(unaff_ss, &local_1a),
    );
    pass1_1008_3e76(
        CONCAT22(unaff_ss, &local_18),
        local_1e,
        local_1c - 1,
        local_1a - 1,
    );
    pass1_1028_87f0(
        CONCAT22(unaff_ss, &local_142),
        0,
        0,
        0xd,
        &local_18,
        unaff_ss,
        local_12,
        _local_6,
    );
    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_142));
    pass1_1008_3e76(
        CONCAT22(unaff_ss, &local_18),
        local_1e,
        local_1c + 1,
        local_1a + 1,
    );
    pass1_1028_87f0(
        CONCAT22(unaff_ss, &local_266),
        0,
        0,
        0xc,
        &local_18,
        unaff_ss,
        local_12,
        _local_6,
    );
    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_266));
    pass1_1008_3e76(
        CONCAT22(unaff_ss, &local_18),
        local_1e,
        local_1c + 1,
        local_1a - 1,
    );
    pass1_1028_87f0(
        CONCAT22(unaff_ss, &local_38a),
        0,
        0,
        0xe,
        &local_18,
        unaff_ss,
        local_12,
        _local_6,
    );
    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_38a));
    pass1_1008_3e54(
        CONCAT22(unaff_ss, &local_390),
        local_1e,
        local_1c - 1,
        local_1a + 1,
    );
    pass1_1028_87f0(
        CONCAT22(unaff_ss, &local_4b4),
        0,
        0,
        0xb,
        &local_390,
        unaff_ss,
        local_12,
        _local_6,
    );
    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_4b4));
    pass1_1008_3e76(
        CONCAT22(unaff_ss, &local_18),
        local_1e,
        local_1c - 1,
        local_1a,
    );
    pass1_1028_87f0(
        CONCAT22(unaff_ss, &local_5d8),
        0,
        0,
        0x7a,
        &local_18,
        unaff_ss,
        local_12,
        _local_6,
    );
    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_5d8));
    pass1_1008_3e76(
        CONCAT22(unaff_ss, &local_18),
        _local_1e,
        (_local_1e >> 0x10),
        local_1a + 1,
    );
    pass1_1028_87f0(
        CONCAT22(unaff_ss, &local_6fc),
        0,
        0,
        0x7a,
        &local_18,
        unaff_ss,
        local_12,
        _local_6,
    );
    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_6fc));
    pass1_1008_3e76(
        CONCAT22(unaff_ss, &local_18),
        local_1e,
        local_1c + 1,
        local_1a,
    );
    pass1_1028_87f0(
        CONCAT22(unaff_ss, &local_820),
        0,
        0,
        0x7a,
        &local_18,
        unaff_ss,
        local_12,
        _local_6,
    );
    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_820));
    pass1_1008_3e76(
        CONCAT22(unaff_ss, &local_18),
        _local_1e,
        (_local_1e >> 0x10),
        local_1a - 1,
    );
    pass1_1028_87f0(
        CONCAT22(unaff_ss, &local_944),
        0,
        0,
        0x7a,
        &local_18,
        unaff_ss,
        local_12,
        _local_6,
    );
    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_944));
    pu_var3 = &local_390;
    pass1_1030_627e(_PTR_LOOP_1050_5740, CONCAT22(unaff_ss, pu_var3), _local_6);
    u_var5 = (local_e >> 0x10);
    (local_e + 0x10) = pu_var3;
    (local_e + 0x12) = extraout_dx;
    return;
}

pub fn pass1_1028_3246(param_1: *mut Struct44) {
    let u_var1: u8;
    let extraout_var: u32;
    let mut in_dx: i32;
    let mut unaff_ss: u16;
    let mut u_var2: u16;
    let mut u_var3: u16;
    let mut u_var4: u16;
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

    u_var1 = pass1_1028_b58e(param_1);
    _local_6 = CONCAT31(extraout_var, u_var1) & 0xffff | in_dx << 0x10;
    local_a = (CONCAT31(extraout_var, u_var1) + 0x2e);
    local_e = (local_a + 0x10);
    u_var3 = 0;
    u_var4 = 1;
    u_var2 = 1;
    local_12 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_e, (local_e >> 0x10));
    local_10 = in_dx;
    pass1_1030_7c50(CONCAT22(in_dx, local_12), CONCAT22(u_var3, u_var2), u_var4);
    pass1_1030_7c50(CONCAT22(local_10, local_12), 1, 2);
    pass1_1030_7c50(CONCAT22(local_10, local_12), 1, 3);
    pass1_1030_7c50(CONCAT22(local_10, local_12), 1, 5);
    _local_1a =
        process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000ffd8, 2));
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
        pass1_1020_a43e(CONCAT22(unaff_ss, local_20));
        pass1_1020_a89e(
            CONCAT22(unaff_ss, local_20),
            (_local_6 + 0xc),
            (_local_6 >> 0x10),
        );
    }
    return;
}

pub fn pass1_1028_33f6(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_3484(param_1: *mut Struct727) -> *mut Struct763 {
    pass1_1028_0068(param_1);
    param_1 = 0x34f6;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_34a6(
    param_1: *mut Struct728,
    param_2: u16,
    param_3: u16,
    param_3_00: u32,
) -> *mut Struct728 {
    pass1_1028_00cc(
        param_1,
        CONCAT22(param_2, param_1._2_2_),
        CONCAT22(param_3_00, param_3),
    );
    param_1.field_0x0 = 0x34f6;
    (param_1).field_0x2 = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_34d0(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1028_0138(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_355e(param_1: *mut Struct763) -> *mut Struct763 {
    pass1_1028_b354(param_1);
    param_1.field_0x0 = 0x3608;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_3580(
    param_1: *mut Struct761,
    param_2: u16,
    param_3: u16,
    param_3_00: u32,
) -> *mut Struct761 {
    pass1_1028_b39e(param_1, param_2, CONCAT22(param_3_00, param_3));
    param_1.field_0x0 = 0x3608;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_35b0(param_1: *mut Struct44, param_2: i32) {
    let u_var1: u8;
    let extraout_AH: u8;
    let mut in_dx: u16;
    let mut u_var2: u16;

    u_var1 = pass1_1028_b58e(param_1);
    if (param_2 == 0) {
        u_var2 = 0;
    } else {
        u_var2 = 0x32;
    }
    pass1_1030_7d1c(
        CONCAT22(in_dx, CONCAT11(extraout_AH, u_var1)),
        u_var2,
        0x230000,
    );
    return;
}

pub fn pass1_1028_35e2(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
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
    param_1: *mut Struct762,
    param_2: u16,
    param_3: u16,
    param_3_00: u32,
) -> *mut Struct762 {
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
    let pi_var1: *mut i32;
    let mut u_var2: i32;
    let mut u_var3: u32;
    let mut u_var4: u32;
    let mut in_dx: i32;
    let mut u_var5: u16;
    let mut local_4: u16;

    param_2 = 0;
    u_var5 = (param_1 >> 0x10);
    if ((param_1 + 0x28) != 0) {
        local_4 = 4;
        while (local_4 < 0x1d) {
            u_var3 = (param_1 + 0x28);
            u_var4 = pass1_1020_bae6(u_var3, CONCAT22(local_4, (u_var3 >> 0x10)));
            u_var2 = param_2;
            param_2 = param_2 + u_var4;
            pi_var1 = (param_2 + 2);
            unsafe {
                *pi_var1 = *pi_var1 + in_dx + CARRY2(u_var2, u_var4);
            }
            if (0xf9 < param_2) {
                return;
            }
            local_4 = local_4 + 1;
        }
    }
    return;
}

pub fn pass1_1028_3718(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1028_388e(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_37a6(param_1: *mut Struct763) {
    let mut u_var1: i32;
    let struct_a: *mut Struct199;
    let paVar2: *mut Struct199;
    let mut extraout_dx: u16;
    let local_bx_13: *mut Struct763;
    let mut u_var3: u16;
    let mut local_4: u16;

    pass1_1028_b354(param_1);
    u_var3 = (param_1 >> 0x10);
    local_bx_13 = param_1;
    u_var1 = 0;
    local_bx_13.field_0x20 = 0;
    local_bx_13.field_0x24 = 0;
    &local_bx_13.field_0x28 = 0;
    param_1.field_0x0 = 0x3e2c;
    local_bx_13.field_0x2 = &PTR_LOOP_1050_1028;
    paVar2 = struct_a;
    process_struct_1000_179c(10, struct_a);
    if ((paVar2 | u_var1) == 0) {
        &local_bx_13.field_0x28 = 0;
    } else {
        pass1_1020_ba3e(CONCAT22(paVar2, u_var1), 5, 5);
        local_bx_13.field_0x28 = u_var1;
        local_bx_13.field_0x2a = extraout_dx;
    }
    return;
}

pub fn pass1_1028_3816(param_1: *mut Struct764, param_2: u32, param_3: u32) {
    let mut u_var1: i32;
    let struct_a: *mut Struct199;
    let paVar2: *mut Struct199;
    let mut extraout_dx: u16;
    let mut u_var3: u16;
    let mut local_4: u16;

    u_var3 = param_2;
    pass1_1028_b39e(CONCAT22(u_var3, param_1), (param_2 >> 0x10), param_3);
    u_var1 = 0;
    param_1.field_0x20 = 0;
    param_1.field_0x24 = 0;
    &param_1.field_0x28 = 0;
    CONCAT22(u_var3, param_1) = 0x3e2c;
    param_1.field_0x2 = &PTR_LOOP_1050_1028;
    paVar2 = struct_a;
    process_struct_1000_179c(10, struct_a);
    if ((paVar2 | u_var1) == 0) {
        &param_1.field_0x28 = 0;
    } else {
        pass1_1020_ba3e(CONCAT22(paVar2, u_var1), 5, 5);
        param_1.field_0x28 = u_var1;
        param_1.field_0x2a = extraout_dx;
    }
    return;
}

pub fn pass1_1028_388e(param_1: *mut Struct44) {
    let mut u_var1: i32;
    let in_struct_1: *mut Struct44;
    let local_bx_4: *mut Struct44;
    let mut u_var2: u16;
    let mut local_6: u32;

    u_var2 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    param_1.ptr_a_lo = 0x3e2c;
    local_bx_4.ptr_a_hi = &PTR_LOOP_1050_1028;
    in_struct_1 = &local_bx_4.field_0x28;
    u_var1 = &local_bx_4.field_0x2a;
    if ((u_var1 | in_struct_1) != 0) {
        pass1_1020_ba7e((in_struct_1 & 0xffff | u_var1 << 0x10));
        error_check_1000_17ce(in_struct_1);
    }
    pass1_1028_b418(param_1);
    return;
}

pub fn pass1_1028_38d4(param_1: *mut u8, param_2: *mut u8, param_3: u32, param_4: u32) {
    let mut i_var1: i32;
    let mut u_var2: u16;
    let fn_ptr_1: fn();

    i_var1 = pass1_1028_c7b6(param_1, param_2, param_4);
    if ((i_var1 == 5) || (i_var1 == 6)) {
        u_var2 = (param_1 >> 0x10);
        fn_ptr_1 = (param_1 + 0x60);
        i_var1 = (**fn_ptr_1)();
        if (i_var1 != 0) {
            pass1_1028_c23e(param_1, u_var2, param_2, param_3, param_4);
            if (i_var1 != 0) {
                u_var2 = pass1_1028_c314(
                    param_1,
                    u_var2,
                    param_2,
                    param_3,
                    (param_3 >> 0x10),
                    param_4,
                );
                if (u_var2 != 0) {
                    return 1;
                }
            }
        }
    } else {
        PTR_LOOP_1050_50ca = 0x6a8;
    }
    return 0;
}

pub fn pass1_1028_3958(param_1: *mut Struct44) {
    let plVar1: *mut long;
    let mut i_var2: i32;
    let Var3: u16;
    let u_var4: u8;
    let mut u_var5: i32;
    let extraout_var: u32;
    let mut u_var6: u32;
    let mut in_dx: u16;
    let mut u_var7: i32;
    let mut u_var8: i32;
    let mut i_var9: i32;
    let mut u_var10: u16;
    let mut unaff_ss: u16;
    let mut u_var11: u16;
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
    let temp_7ff6159465a: *mut Struct768;

    local_4 = in_dx;
    u_var4 = pass1_1028_b58e(param_1);
    local_6 = CONCAT31(extraout_var, u_var4);
    local_a = (local_6 + 0x22);
    u_var8 = (local_6 + 0x24);
    u_var6 = u_var8;
    if ((u_var8 | local_a) != 0) {
        if (PTR_LOOP_1050_574c != 0x0) {
            _local_1e = *(local_a + 4);
            local_22 = 0;
            while (local_22 < _local_1e) {
                pass1_1020_bb16(
                    local_a,
                    CONCAT22(unaff_ss, &local_2c),
                    CONCAT22(unaff_ss, &local_28),
                    local_22,
                );
                local_22 = local_22 + 1;
            }
        }
        local_e = (local_6 + 0x2e);
        local_12 = *ctx._PTR_LOOP_1050_65e2;
        local_14 = local_12 & 1;
        local_16 = 4;
        while (u_var8 = u_var6, local_16 < 0xe) {
            local_2c = local_16;
            u_var6 = pass1_1020_bae6(local_a, CONCAT22(local_16, (local_a >> 0x10)));
            local_28 = u_var6 & 0xffff | u_var8 << 0x10;
            u_var8 = u_var8 | u_var6;
            u_var6 = u_var8;
            if (u_var8 != 0) {
                pass1_1020_bb8a(local_a, 0, 0, local_2c);
                u_var8 = -(local_28._2_2_ + (local_28 != 0));
                u_var6 = u_var8;
                local_22 = CONCAT22(u_var8, -local_28);
                pass1_1038_5694(local_e, CONCAT22(u_var8, -local_28), local_2c);
                _local_1e = 0;
                local_24 = 0;
                i_var9 = param_1;
                u_var10 = (param_1 >> 0x10);
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
                            plVar1 = (i_var9 + 0x20);
                            *plVar1 = *plVar1 + local_28;
                        }
                        u_var8 = (i_var9 + 0x20);
                        u_var5 = (i_var9 + 0x22);
                        u_var7 = u_var8 >> 1 | ((u_var5 & 1) != 0) << 0xf;
                        _local_34 = CONCAT22(u_var5 >> 1, u_var7);
                        u_var7 = (u_var5 & 0xfffe) + CARRY2(u_var7, u_var7);
                        u_var6 = u_var7;
                        (i_var9 + 0x20) = u_var8 - (u_var8 & 0xfffe);
                        (i_var9 + 0x22) = (u_var5 - u_var7) - (u_var8 < (u_var8 & 0xfffe));
                        if (_local_34 != 0) {
                            u_var11 = 0x15;
                            // LAB_1028_3b14:
                            _local_1e = local_28;
                            pass1_1020_bb8a(
                                (i_var9 + 0x28),
                                _local_34,
                                (_local_34 >> 0x10),
                                u_var11,
                            );
                        }
                    }
                    0xd => {
                        local_24 = 0x19;
                        _local_1e = local_28;
                        unsafe {
                            plVar1 = (i_var9 + 0x24);
                            *plVar1 = *plVar1 + local_28;
                        }
                        u_var8 = (i_var9 + 0x24);
                        i_var2 = (i_var9 + 0x26);
                        qVar3 = (local_28 & 0xffff0000 | u_var8) / 3;
                        _local_34 = qVar3;
                        local_32 = (qVar3 >> 0x10);
                        u_var5 = qVar3;
                        u_var7 = local_32 * 3 + CARRY2(u_var5, u_var5) + CARRY2(u_var5 * 2, u_var5);
                        u_var6 = u_var7;
                        (i_var9 + 0x24) = u_var8 + u_var5 * -3;
                        (i_var9 + 0x26) = (i_var2 - u_var7) - (u_var8 < u_var5 * 3);
                        if (_local_34 != 0) {
                            u_var11 = 0x16;
                            // goto LAB_1028_3b14;
                        }
                        if (((local_1c | local_1e) != 0) && (local_24 != 0)) {
                            pass1_1020_bb70(
                                *(i_var9 + 0x28),
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
    let mut i_var1: i32;
    let mut local_6: u16;
    let mut local_4: u16;
    let fn_ptr_1: fn();

    fn_ptr_1 = (param_1 + 0x40);
    i_var1 = (**fn_ptr_1)();
    if (i_var1 != 0) {
        return 0x0;
    }
    return CONCAT22(-(1000 < local_6) - local_4, 1000 - local_6);
}

pub fn pass1_1028_3c60(param_1: *mut Struct770, param_2: u32) {
    let pi_var1: *mut i32;
    let mut u_var2: i32;
    // ppu_var3: *mut *mut u8;
    let mut u_var4: u32;
    let mut in_dx: i32;
    let local_bx_15: *mut Struct770;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut local_10: u32;
    let mut local_c: [u8; 4];
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    param_2 = 0;
    u_var5 = (param_1 >> 0x10);
    local_bx_15 = param_1;
    if (local_bx_15.field_0x28 != 0x0) {
        local_8 = 4;
        while (local_8 < 0x1d) {
            ppu_var3 = local_bx_15.field_0x28;
            u_var4 = pass1_1020_bae6(ppu_var3, CONCAT22(local_8, (ppu_var3 >> 0x10)));
            u_var2 = param_2;
            param_2 = param_2 + u_var4;
            pi_var1 = (param_2 + 2);
            unsafe { *pi_var1 = *pi_var1 + in_dx + CARRY2(u_var2, u_var4) };
            if (999 < param_2) {
                return;
            }
            local_8 = local_8 + 1;
        }
    }
    ppu_var3 = local_bx_15.field_0x28;
    local_4 = (ppu_var3 + 4);
    local_6 = 0;
    while (true) {
        if (local_4 <= local_6) {
            return;
        }
        pass1_1020_bb16(
            local_bx_15.field_0x28,
            CONCAT22(unaff_ss, &local_10),
            CONCAT22(unaff_ss, local_c),
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
    let mut i_var1: i32;
    let mut u_var2: u16;
    let u_var3: u8;
    let mut in_AX: u16;
    let extraout_AH: u8;
    let paVar4: *mut Struct493;
    let b_var5: bool;
    let mut in_dx: u16;
    let mut u_var6: i32;
    let mut unaff_ss: u16;
    let mut u_var7: u32;
    let mut u_var8: u32;
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
    local_8 = pass1_1030_2fac(CONCAT22(in_dx, in_AX));
    _local_c = pass1_1028_bb24(param_1);
    local_e = (_local_c >> 0x10);
    u_var3 = pass1_1028_b58e(param_1);
    local_10 = CONCAT11(extraout_AH, u_var3);
    local_16 = (local_10 + 0xc);
    local_1a = 1;
    while (true) {
        if (local_8 < local_1a) {
            return 0;
        }
        local_12 = local_1a;
        u_var7 = pass1_1030_627e(_PTR_LOOP_1050_5740, CONCAT22(unaff_ss, &local_16), _local_c);
        u_var6 = (u_var7 >> 0x10);
        paVar4 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var7, u_var6);
        if ((u_var6 | paVar4) == 0) {
            return 0;
        }
        u_var8 = pass1_1030_73a8(CONCAT22(u_var6, paVar4));
        u_var2 = (u_var8 >> 0x10);
        if (u_var8 == 0) {
            return 0;
        }
        i_var1 = (u_var8 + 0xc);
        b_var5 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, i_var1, 0x13);
        if ((b_var5 == 0) && (i_var1 != 0x75)) {
            break;
        }
        if ((u_var8 + 0x12) != 9) {
            return 1;
        }
        local_1a = local_1a + 1;
    }
    return 0;
}

pub fn pass1_1028_1646(param_1: *mut Struct750) -> *mut Struct750 {
    let paVar1: *mut Struct750;
    let local_bx_3: *mut Struct750;
    let local_es_3: *mut Struct750;

    local_es_3 = (param_1 >> 0x10);
    local_bx_3 = param_1;
    paVar1 = (local_bx_3.field_0x20 - 4);
    if (paVar1 < &BYTE_1050_0009) {
        match (paVar1) {
            0x0 => {
                local_bx_3.field_0x20 = 5;
            }
            0x1 => {
                local_bx_3.field_0x20 = 6;
            }
            0x2 => {
                local_bx_3.field_0x20 = 7;
            }
            0x3 => {
                local_bx_3.field_0x20 = 8;
            }
            0x4 => {
                local_bx_3.field_0x20 = 9;
            }
            0x5 => {
                local_bx_3.field_0x20 = 10;
                return local_bx_3;
            }
            0x6 => {
                local_bx_3.field_0x20 = 0xb;
                return local_bx_3;
            }
            0x7 => {
                local_bx_3.field_0x20 = 0xc;
                return local_bx_3;
            }
            0x8 => {
                local_bx_3.field_0x20 = 0xd;
                return local_bx_3;
            }
        }
        return local_bx_3;
    }
    local_bx_3.field_0x20 = 4;
    return paVar1;
}

pub fn pass1_1028_16fe(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
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
    let u_var1: u8;
    let extraout_var: u32;
    let mut extraout_dx: u16;
    let mut u_var2: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1030_df0c(param_1, param_2, param_3);
    u_var2 = extraout_dx;
    u_var1 = pass1_1028_b58e(CONCAT22(param_2, param_1));
    pass1_1038_57dc(
        *(CONCAT31(extraout_var, u_var1) + 0x2e),
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
    let u_var1: u8;
    let mut in_AX: i32;
    let mut u_var2: u16;
    let pu_var3: *mut u32;
    let pu_var4: *mut u32;
    let extraout_var: u32;
    let mut extraout_dx: u16;
    let mut unaff_si: u16;
    let mut unaff_ss: u16;
    let mut u_var5: u32;
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

    u_var2 = (param_1 >> 0x10);
    pass1_1028_c3aa(param_1, u_var2, param_2, param_3, param_4);
    if (in_AX == 0) {
        return;
    }
    u_var2 = pass1_1028_c314(
        param_1,
        u_var2,
        param_2,
        param_3,
        (param_3 >> 0x10),
        param_4,
    );
    if (u_var2 == 0) {
        return;
    }
    pu_var3 = &local_c;
    pass1_1030_64ce(_PTR_LOOP_1050_5740, param_2, param_4, pu_var3, unaff_ss);
    unsafe { local_6 = *pu_var3 };
    local_1e = (pu_var3 + 2);
    local_8 = (param_2 + 4);
    _local_16 = (local_6 & 0xffff | local_1e << 0x10);
    local_20 = local_6;
    local_1e._1_1_ = (local_1e >> 8);
    if (local_1e._1_1_ != '\0') {
        local_20 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_6, local_1e);
        u_var1 = pass1_1030_6fa0(CONCAT22(local_1e, local_20));
        local_1c = CONCAT31(extraout_var, u_var1);
        if (local_1c == 0x10) {
            if (local_8 != 0) {
                PTR_LOOP_1050_50ca = 0x6ab;
                return;
            }
            return;
        }
        if ((local_1c == 0x60) || (local_1c == 0x61)) {
            _local_16 =
                process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x2f));
            u_var5 = pass1_1018_04b8(_local_16);
            local_22 = (u_var5 >> 0x10);
            local_10 = u_var5;
            local_18 = (_local_16 + 0x1e);
            local_e = local_22;
            local_24 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_10, local_22);
            u_var2 = pass1_1030_2fac(CONCAT22(local_22, local_24));
            if (u_var2 <= local_18) {
                PTR_LOOP_1050_50ca = 0x6ac;
                return;
            }
            local_2a = param_2;
            local_26 = local_8 + 1;
            pu_var4 = &local_2a;
            pass1_1028_c7b6(param_1, pu_var4, unaff_ss, param_4);
            if (pu_var4 == 0x0) {
                return;
            }
            if (pu_var4 == (&PTR_DAT_0005_0000_1050_0004 + 2)) {
                return;
            }
            return;
        }
    }
    PTR_LOOP_1050_50ca = 0x6aa;
    return;
}

pub fn pass1_1028_199a(param_1: *mut Struct44) {
    let pi_var1: *mut i32;
    let u_var2: u8;
    let local_AX__1: *mut Struct751;
    let extraout_var: u32;
    let mut in_dx: u16;
    let mut unaff_si: u16;
    let mut u_var3: u16;
    let mut unaff_ss: u16;
    let pu_var4: *mut u16;
    let mut u_var5: u16;
    let puVar6: *mut u16;
    let mut u_var7: u16;
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
    let mut u_stack12: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    pi_var1 = (param_1 + 0x14);
    unsafe { *pi_var1 = *pi_var1 + -1 };
    let pi_var1_val = unsafe { *pi_var1 };
    if (pi_var1_val == 0) {
        u_var2 = pass1_1028_b58e(param_1);
        local_6 = CONCAT31(extraout_var, u_var2);
        local_a = *(local_6 + 0x2e);
        local_4 = in_dx;
        pass1_1038_5804(local_a, 1, (&dos_alloc_addr_1050_0002 + 1));
        local_10 = (local_6 + 0xc);
        u_stack12 = (local_6 + 0x10);
        local_32 = &local_10;
        pass1_1008_3eb4(
            CONCAT22(unaff_ss, &local_10),
            CONCAT22(unaff_ss, &local_16),
            CONCAT22(unaff_ss, &local_14),
            CONCAT22(unaff_ss, &local_14 + 2),
        );
        _local_1a = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x2f));
        local_1e = (_local_1a + 0x20);
        puVar6 = &local_20;
        local_24 = &local_22;
        pu_var4 = local_24;
        u_var5 = unaff_ss;
        u_var7 = unaff_ss;
        local_26 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_1e, (local_1e >> 0x10));
        pass1_1030_5b1c(
            CONCAT22(local_24, local_26),
            CONCAT22(u_var5, pu_var4),
            CONCAT22(u_var7, puVar6),
        );
        if (local_22 < (local_16 + 1)) {
            pass1_1030_5b3e(CONCAT22(local_24, local_26), local_16 + 1, local_20);
            pass1_1030_5b1c(
                CONCAT22(local_24, local_26),
                CONCAT22(unaff_ss, &local_22),
                CONCAT22(unaff_ss, &local_20),
            );
        }
        u_var3 = (local_a >> 0x10);
        local_2a = (local_a + 4);
        pass1_1028_87f0(
            CONCAT22(unaff_ss, &local_15e),
            0,
            0,
            (-(local_16 == 0) & 0xffd3) + 0x60,
            &local_10,
            unaff_ss,
            local_2a & 0xffff | *(local_a + 6) << 0x10,
            local_1e,
        );
        pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_15e));
        local_15e = ctx.s_1_1050_389a;
        local_15c = &ctx.PTR_LOOP_1050_1008;
        pass1_1008_3e76(
            CONCAT22(unaff_ss, &local_10),
            local_16 + 1,
            local_14,
            (local_14 >> 0x10),
        );
        pass1_1028_87f0(
            CONCAT22(unaff_ss, &local_15e),
            0,
            0,
            0x60,
            &local_10,
            unaff_ss,
            local_2a,
            local_1e,
        );
        pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_15e));
    }
    return;
}

pub fn pass1_1028_1b1e(param_1: u32) {
    (param_1 + 0x14) = 7;
    return;
}

pub fn pass1_1028_1b2e(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1030_dcf4(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_1bbc(param_1: *mut Struct752) -> *mut Struct752 {
    let local_bx_14: *mut Struct752;
    let mut u_var1: u16;

    pass1_1028_b354(param_1);
    u_var1 = (param_1 >> 0x10);
    local_bx_14 = param_1;
    local_bx_14.field_0x20 = 0;
    local_bx_14.field_0x22 = 0;
    param_1 = (s_526_bmp_1050_1ee7 + 7);
    local_bx_14.field_0x2 = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_1be8(param_1: *mut Struct753, param_2: u32, param_3: u32) {
    let mut u_var1: u16;

    u_var1 = param_2;
    pass1_1028_b39e(CONCAT22(u_var1, param_1), (param_2 >> 0x10), param_3);
    param_1.field_0x20 = 0;
    param_1.field_0x22 = 0;
    CONCAT22(u_var1, param_1) = (s_526_bmp_1050_1ee7 + 7);
    param_1.field_0x2 = &PTR_LOOP_1050_1028;
    return CONCAT22(u_var1, param_1);
}

pub fn return_false_1028_1c1c() -> bool {
    return 0;
}

pub fn pass1_1028_1c22(param_1: *mut Struct754) {
    let mut u_var1: u16;
    let local_bx_3: *mut Struct754;
    let mut u_var2: i32;

    u_var2 = (param_1 >> 0x10);
    local_bx_3 = param_1;
    if ((local_bx_3.field_0x20 != 0)
        && (local_bx_3.field_0x12 == 5 || (local_bx_3.field_0x12 == 6)))
    {
        if (local_bx_3.field_0xc == 0x16) {
            return 0x19;
        }
        if (local_bx_3.field_0xc == 0x17) {
            return 0x1a;
        }
    }
    u_var1 = pass1_1028_bc1c((param_1 & 0xffff | u_var2 << 0x10));
    return u_var1;
}

pub fn pass1_1028_1c66(param_1: *mut *mut Struct758) {
    let pp_var1: fn();
    let mut i_var2: i32;
    let local_bx_4: *mut Struct755;
    let mut u_var3: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    local_bx_4 = param_1;
    if (local_bx_4.field_0x12 != 6) {
        return;
    }
    u_var3 = pass1_1028_b4f2(param_1);
    if ((u_var3 + 0x200) != 0x8000002) {
        pp_var1 = (param_1 + 100);
        i_var2 = (**pp_var1)();
        if (i_var2 == 0) {
            return;
        }
        i_var2 = pass1_1028_cb04(param_1);
        if (i_var2 == 0) {
            i_var2 = 6;
            // goto LAB_1028_1cbd;
        }
        pass1_1028_c952(param_1);
    }
    i_var2 = 5;
    // LAB_1028_1cbd:
    pass1_1028_bdac(param_1, i_var2);
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
    let b_var1: bool;
    let mut unaff_ss: u16;
    let mut u_var2: u16;
    let mut u_var3: u16;
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
        CONCAT22(unaff_ss, &local_8),
        CONCAT22(unaff_ss, local_e),
        CONCAT22(unaff_ss, &local_c),
        CONCAT22(unaff_ss, &local_a),
    );
    _local_8 = _local_8 & 0xffff | (local_c - 1) << 0x10;
    u_var2 = param_1;
    u_var3 = (param_1 >> 0x10);
    b_var1 = pass1_1028_1e14(
        u_var2,
        u_var3,
        0x16,
        CONCAT22(unaff_ss, &local_8),
        param_4_00,
    );
    if (b_var1 == 0) {
        _local_8 = _local_8 & 0xffff | (local_c + 1) << 0x10;
        b_var1 = pass1_1028_1e14(
            u_var2,
            u_var3,
            0x16,
            CONCAT22(unaff_ss, &local_8),
            param_4_00,
        );
        if (b_var1 == 0) {
            local_8 = local_a - 1;
            local_6 = local_c;
            b_var1 = pass1_1028_1e14(
                u_var2,
                u_var3,
                0x17,
                CONCAT22(unaff_ss, &local_8),
                param_4_00,
            );
            if (b_var1 == 0) {
                _local_8 = CONCAT22(local_6, local_a + 1);
                b_var1 = pass1_1028_1e14(
                    u_var2,
                    u_var3,
                    0x17,
                    CONCAT22(unaff_ss, &local_8),
                    param_4_00,
                );
                if (b_var1 == 0) {
                    return b_var1;
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
    let paVar2: *mut Struct493;
    let pu_var3: *mut u8;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let mut u_var5: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: [u8; 2];

    u_var5 = pass1_1030_bcae(local_4, unaff_ss);
    u_var4 = (u_var5 >> 0x10);
    paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, param_2_00, (param_2_00 >> 0x10));
    u_var1 = &paVar2.field_0x10;
    paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
    pu_var3 = local_4;
    pass1_1030_bcde(
        pu_var3,
        unaff_ss,
        CONCAT22(u_var4, paVar2),
        param_1_00,
        param_5,
    );
    if (pu_var3 < 0) {
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
    let mut i_var1: i32;
    let paVar2: *mut Struct493;
    let mut u_var3: i32;
    let mut u_var4: i32;
    let lVar5: u32;
    let mut u_var6: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_a: u16;
    let mut local_4: u16;

    lVar5 = pass1_1030_627e(_PTR_LOOP_1050_5740, param_2_00, param_5);
    u_var3 = (lVar5 >> 0x10);
    u_var4 = u_var3 | lVar5;
    if (lVar5 != 0) {
        paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, lVar5, u_var3);
        if ((u_var4 | paVar2) != 0) {
            u_var6 = pass1_1030_73a8(CONCAT22(u_var4, paVar2));
            if (u_var6 != 0) {
                i_var1 = (u_var6 + 0xc);
                if (((i_var1 == 0x18) || (i_var1 == 0x3f)) || (i_var1 == param_1_00)) {
                    return 1;
                }
            }
        }
    }
    return 0;
}

pub fn pass1_1028_1e8a(param_1: u32) {
    let u_var1: u8;
    let extraout_AH: u8;
    let mut in_dx: u16;
    let mut u_var2: i32;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let mut u_var5: u16;

    u_var2 = (param_1 >> 0x10);
    if ((*(param_1 + 0x1a) & 2) == 0) {
        u_var4 = 0;
        u_var5 = 0x23;
        u_var3 = 1;
        u_var1 = pass1_1028_b58e((param_1 & 0xffff | u_var2 << 0x10));
        pass1_1030_7d7c(
            CONCAT22(in_dx, CONCAT11(extraout_AH, u_var1)),
            u_var3,
            CONCAT22(u_var5, u_var4),
        );
        pass1_1028_bdac(param_1, 6);
        return 0;
    }
    return 1;
}

pub fn pass1_1028_1ec8(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: This function may have set the stack p i32er

pub fn pass1_1028_1eee(param_1: u32, param_2: u8) {
    let pc_var1: *mut libc::c_char;
    let pb_var2: *mut u8;
    let mut bVar3: u8;
    let mut bVar4: u8;
    let mut b_var5: u8;
    let mut cVar6: u8;
    let pu_var7: *mut u8;
    let mut bVar8: u8;
    let mut in_dx: u16;
    let mut bVar9: u8;
    let pu_var11: *mut u16;
    let unaff_BP: *mut u16;
    let unaff_si: *mut u8;
    let mut unaff_DI: i32;
    let mut unaff_cs: u16;
    let mut unaff_ss: u16;
    let mut bVar12: bool;
    let mut bVar13: bool;
    let mut i_var10: i32;

    pu_var11 = &stack0xfffe;
    cVar6 = '\x0f';
    while {
        unaff_BP = unaff_BP + -1;
        pu_var11 = pu_var11 + -1;
        unsafe { *pu_var11 = *unaff_BP };
        pu_var7 = PTR_LOOP_1050_1028;
        cVar6 = cVar6 + -1;
        '\0' < cVar6
    } {}
    pb_var2 = unaff_si + 0x1028;
    bVar8 = in_dx;
    unsafe {
        *pb_var2 = *pb_var2 ^ bVar8;
        _in(in_dx);
        pc_var1 = &stack0xfffe + unaff_si;
        *pc_var1 = *pc_var1 + '(';
        bVar9 = *unaff_si & 0x28;
        i_var10 = CONCAT11(0x10, bVar9);
        pb_var2 = unaff_si + i_var10;
        *pb_var2 = *pb_var2 - bVar8;
        if ('\x10' < *(&PTR_LOOP_1050_1028 + unaff_DI)) {
            pb_var2 = unaff_si + i_var10;
            bVar3 = *pb_var2;
            *pb_var2 = *pb_var2 - bVar8;
            pb_var2 = unaff_si + i_var10;
            bVar4 = *pb_var2;
            *pb_var2 = *pb_var2 - bVar8;
            b_var5 = (bVar3 < bVar8) + 0xb5;
            bVar12 = ((bVar3 < bVar8) - 0x23) < 0x28 || b_var5 < (bVar4 < bVar8);
            bVar13 = CARRY1(s_fem79_wav_1050_28ba[5], bVar8);
            bVar3 = s_fem79_wav_1050_28ba[5] + bVar8;
            s_fem79_wav_1050_28ba[5] = bVar3 + bVar12;
            pc_var1 = &stack0xfffe + unaff_si;
            *pc_var1 = *pc_var1 + bVar9 + (bVar13 || CARRY1(bVar3, bVar12));
            0x1026 = &ctx.g_alloc_addr_1050_1050;
            LOCK();
            pc_var1 = (&PTR_LOOP_1050_1028 + CONCAT11(s_501a_bmp_1050_2050[0] + '\x10', 0x28));
            *pc_var1 = *pc_var1 - bVar8;
            return CONCAT22(in_dx, CONCAT11(0x10, b_var5 - (bVar4 < bVar8)));
        }
    }
    0x1024 = param_1;
    0x1022 = unaff_cs;
    0x1020 = (s_523_bmp_1050_1ecf + 6);
    pass1_1028_b418(paRam00001024);
    if ((param_2 & 1) != 0) {
        0x1024 = param_1;
        0x1022 = unaff_cs;
        0x1020 = (s_525_bmp_1050_1edf + 5);
        error_check_1000_17ce(paRam00001024);
    }
    return param_1;
}

pub fn pass1_1028_1f56(param_1: *mut Struct756) {
    let mut u_var1: u32;
    let mut u_var2: i32;
    let paVar3: *mut Struct199;
    let struct_a: *mut Struct199;
    let mut u_var4: i32;
    let local_bx_13: *mut Struct756;
    let mut u_var5: u16;
    let mut local_4: u16;

    pass1_1028_b354(param_1);
    u_var5 = (param_1 >> 0x10);
    local_bx_13 = param_1;
    u_var2 = 0;
    &local_bx_13.field_0x20 = 0;
    local_bx_13.field_0x24 = 0;
    param_1 = 0x2572;
    local_bx_13.field_0x2 = &PTR_LOOP_1050_1028;
    paVar3 = struct_a;
    process_struct_1000_179c(0xc, struct_a);
    u_var4 = paVar3 | u_var2;
    if (u_var4 == 0) {
        &local_bx_13.field_0x20 = 0;
    } else {
        paVar3 = process_struct_1008_574a(CONCAT22(paVar3, u_var2));
        local_bx_13.field_0x20 = paVar3;
        &local_bx_13.field_0x22 = u_var4;
    }
    u_var1 = &local_bx_13.field_0x20;
    (u_var1 + 10) = 0;
    return;
}

pub fn pass1_1028_1fc8(param_1: *mut Struct757, param_2: u32, param_3: u32) {
    let mut u_var1: u32;
    let mut u_var2: i32;
    let paVar3: *mut Struct199;
    let struct_a: *mut Struct199;
    let mut u_var4: i32;
    let mut local_4: u16;

    pass1_1028_b39e(CONCAT22(param_2, param_1), (param_2 >> 0x10), param_3);
    u_var2 = 0;
    &param_1.field_0x20 = 0;
    param_1.field_0x24 = 0;
    CONCAT22(param_2, param_1) = 0x2572;
    param_1.field_0x2 = &PTR_LOOP_1050_1028;
    paVar3 = struct_a;
    process_struct_1000_179c(0xc, struct_a);
    u_var4 = paVar3 | u_var2;
    if (u_var4 == 0) {
        &param_1.field_0x20 = 0;
    } else {
        paVar3 = process_struct_1008_574a(CONCAT22(paVar3, u_var2));
        param_1.field_0x20 = paVar3;
        &param_1.field_0x22 = u_var4;
    }
    u_var1 = &param_1.field_0x20;
    (u_var1 + 10) = 0;
    return;
}

pub fn pass1_1028_2042(param_1: *mut Struct44) {
    let pu_var1: *mut u32;
    let ppc_var2: fn();
    let mut extraout_dx: i32;
    let local_bx_4: *mut Struct44;
    let mut local_es_4: u16;
    let mut local_DXAX_81: u32;
    let fn_ptr_1: *mut u32;
    let mut temp_5f32db2c2f: u32;

    local_es_4 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    param_1.ptr_a_lo = 0x2572;
    local_bx_4.ptr_a_hi = &PTR_LOOP_1050_1028;
    temp_5f32db2c2f = &local_bx_4.field_0x20;
    (temp_5f32db2c2f + 10) = 1;
    pu_var1 = local_bx_4.field_0x20;
    local_DXAX_81._2_2_ = &local_bx_4.field_0x22;
    if ((local_DXAX_81._2_2_ | pu_var1) != 0) {
        unsafe {
            ppc_var2 = *pu_var1;
            ppc_var2();
        }
        local_DXAX_81._2_2_ = extraout_dx;
    }
    if ((ctx._PTR_LOOP_1050_65e2 != 0) && (_PTR_LOOP_1050_5a64 != 0x0)) {
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

pub fn pass1_1028_20b6(param_1: *mut Struct44) {
    let u_var1: u8;
    let local_AH_46: u8;
    let BVar3: bool;
    let mut local_DX_31: u16;
    let mut u_var4: u16;
    let mut u_var5: i32;
    let mut local_SS__1: u16;
    let mut local_16: [u8; 2];
    let mut iStack20: i32;
    let mut iStack18: i32;
    let mut uStack16: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut i_var2: i32;

    pass1_1028_be9e(param_1);
    u_var5 = (param_1 >> 0x10);
    u_var4 = param_1;
    if ((u_var4 + 0x12) == 5) {
        _local_6 = pass1_1028_bb24((param_1 & 0xffff | u_var5 << 0x10));
        local_DX_31 = (_local_6 >> 0x10);
        u_var1 = pass1_1028_b58e(param_1);
        i_var2 = CONCAT11(local_AH_46, u_var1);
        _local_a = CONCAT22(local_DX_31, i_var2);
        uStack16 = (i_var2 + 0xc);
        local_c = (i_var2 + 0x10);
        pass1_1008_3eb4(
            CONCAT22(local_SS__1, &uStack16),
            CONCAT22(local_SS__1, local_16),
            CONCAT22(local_SS__1, &iStack20),
            CONCAT22(local_SS__1, &iStack18),
        );
        uStack16 = uStack16 & 0xffff | (iStack20 - 1) << 0x10;
        BVar3 = pass1_1028_21ba(u_var4, u_var5, CONCAT22(local_SS__1, &uStack16), _local_6);
        if (BVar3 == 0) {
            uStack16 = uStack16 & 0xffff | (iStack20 + 1) << 0x10;
            BVar3 = pass1_1028_21ba(u_var4, u_var5, CONCAT22(local_SS__1, &uStack16), _local_6);
            if (BVar3 == 0) {
                uStack16 = CONCAT22(iStack20, iStack18 + -1);
                BVar3 = pass1_1028_21ba(u_var4, u_var5, CONCAT22(local_SS__1, &uStack16), _local_6);
                if (BVar3 == 0) {
                    uStack16 = uStack16 & 0xffff0000 | (iStack18 + 1);
                    BVar3 =
                        pass1_1028_21ba(u_var4, u_var5, CONCAT22(local_SS__1, &uStack16), _local_6);
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
    let paVar1: *mut Struct493;
    let mut u_var2: i32;
    let mut u_var3: i32;
    let lVar4: u32;
    let mut u_var5: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_a: u16;
    let mut local_4: u16;

    lVar4 = pass1_1030_627e(_PTR_LOOP_1050_5740, param_1_00, param_2_00);
    u_var2 = (lVar4 >> 0x10);
    u_var3 = u_var2 | lVar4;
    if (lVar4 != 0) {
        paVar1 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, lVar4, u_var2);
        if ((u_var3 | paVar1) != 0) {
            u_var5 = pass1_1030_73a8(CONCAT22(u_var3, paVar1));
            if ((u_var5 != 0) && ((u_var5 + 0xc) == 0x40)) {
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
    let mut i_var1: i32;
    let paVar2: *mut Struct493;
    let mut u_var3: i32;
    let mut u_var4: i32;
    let lVar5: u32;
    let mut u_var6: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_a: u16;
    let mut local_4: u16;

    lVar5 = pass1_1030_627e(_PTR_LOOP_1050_5740, param_2_00, param_5);
    u_var3 = (lVar5 >> 0x10);
    u_var4 = u_var3 | lVar5;
    if (lVar5 != 0) {
        paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, lVar5, u_var3);
        if ((u_var4 | paVar2) != 0) {
            u_var6 = pass1_1030_73a8(CONCAT22(u_var4, paVar2));
            if ((u_var6 != 0)
                && ((
                    i_var1 = (u_var6 + 0xc),
                    i_var1 == 0x40 || (i_var1 == param_1_00),
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
    let mut u_var1: u16;
    let mut unaff_ss: u16;
    let mut u_var2: u16;
    let mut u_var3: u16;
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
        CONCAT22(unaff_ss, &local_8),
        CONCAT22(unaff_ss, local_e),
        CONCAT22(unaff_ss, &local_c),
        CONCAT22(unaff_ss, &local_a),
    );
    _local_8 = _local_8 & 0xffff | (local_c - 1) << 0x10;
    u_var2 = param_1;
    u_var3 = (param_1 >> 0x10);
    u_var1 = pass1_1028_2220(
        u_var2,
        u_var3,
        0x16,
        CONCAT22(unaff_ss, &local_8),
        param_4_00,
    );
    if (u_var1 == 0) {
        _local_8 = _local_8 & 0xffff | (local_c + 1) << 0x10;
        u_var1 = pass1_1028_2220(
            u_var2,
            u_var3,
            0x16,
            CONCAT22(unaff_ss, &local_8),
            param_4_00,
        );
        if (u_var1 == 0) {
            local_8 = local_a - 1;
            local_6 = local_c;
            u_var1 = pass1_1028_2220(
                u_var2,
                u_var3,
                0x17,
                CONCAT22(unaff_ss, &local_8),
                param_4_00,
            );
            if (u_var1 == 0) {
                _local_8 = CONCAT22(local_6, local_a + 1);
                u_var1 = pass1_1028_2220(
                    u_var2,
                    u_var3,
                    0x17,
                    CONCAT22(unaff_ss, &local_8),
                    param_4_00,
                );
                if (u_var1 == 0) {
                    return u_var1;
                }
            }
        }
    }
    return 1;
}

pub fn pass1_1028_236a(param_1: *mut Struct44) -> bool {
    let u_var1: u8;
    let extraout_AH: u8;
    let mut in_dx: u16;
    let mut u_var2: i32;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let mut u_var5: u16;

    u_var2 = (param_1 >> 0x10);
    if ((*(param_1 + 0x1a) & 2) == 0) {
        u_var4 = 0;
        u_var5 = 0x23;
        u_var3 = 1;
        u_var1 = pass1_1028_b58e((param_1 & 0xffff | u_var2 << 0x10));
        pass1_1030_7d7c(
            CONCAT22(in_dx, CONCAT11(extraout_AH, u_var1)),
            u_var3,
            CONCAT22(u_var5, u_var4),
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
    let paVar2: *mut Struct493;
    let pu_var3: *mut u8;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let mut u_var5: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: [u8; 2];

    u_var5 = pass1_1030_bcae(local_4, unaff_ss);
    u_var4 = (u_var5 >> 0x10);
    paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, param_2_00, (param_2_00 >> 0x10));
    u_var1 = &paVar2.field_0x10;
    paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
    pu_var3 = local_4;
    pass1_1030_bcde(
        pu_var3,
        unaff_ss,
        CONCAT22(u_var4, paVar2),
        param_1_00,
        param_5,
    );
    if (pu_var3 < 0) {
        PTR_LOOP_1050_50ca = 0x6af;
        return;
    }
    return;
}

pub fn pass1_1028_04ee(param_1: u32, param_2: u32) {
    let pi_var1: *mut i32;
    let mut u_var2: i32;
    let mut u_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let lVar5: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    param_2 = 0;
    pass1_1008_5784(CONCAT22(unaff_ss, local_a), (param_1 + 0x22));
    while {
        lVar5 = pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
        if (lVar5 == 0) {
            return 0;
        }
        u_var2 = (lVar5 + 0xc);
        u_var4 = (param_2 >> 0x10);
        u_var3 = param_2;
        param_2 = param_2 + u_var2;
        pi_var1 = (param_2 + 2);
        unsafe { *pi_var1 = *pi_var1 + CARRY2(u_var3, u_var2) };
        ((param_2 + 2) == 0) && (param_2 < 0x1e)
    } {}
    return 1;
}

pub fn pass1_1028_0550(param_1: u32) {
    let u_var1: u8;
    let extraout_AH: u8;
    let mut extraout_dx: u16;
    let mut u_var2: u16;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let mut u_var5: i32;
    let mut u_var6: u16;

    u_var5 = (param_1 >> 0x10);
    pass1_1028_be9e(param_1, u_var5);
    if ((param_1 + 0x12) == 5) {
        u_var4 = 0;
        u_var6 = 4;
        u_var3 = 1;
        u_var2 = extraout_dx;
        u_var1 = pass1_1028_b58e((param_1 & 0xffff | u_var5 << 0x10));
        pass1_1030_7c50(
            CONCAT22(u_var2, CONCAT11(extraout_AH, u_var1)),
            CONCAT22(u_var4, u_var3),
            u_var6,
        );
    }
    return;
}

pub fn pass1_1028_0582(param_1: *mut Struct734) {
    let pu_var1: *mut u32;
    let plVar2: *mut long;
    let mut u_var3: u32;
    let ppcVar4: fn();
    let mut u_var5: u32;
    let u_var6: u8;
    let local_AX__1: *mut Struct738;
    let pu_var7: *mut u8;
    let local_AX_150: *mut Struct733;
    let puVar8: *mut u16;
    let local_AX_402: *mut Struct736;
    let mut u_var9: i32;
    let mut u_var10: u16;
    let extraout_var: u32;
    let mut in_dx: i32;
    let mut extraout_dx: i32;
    let mut extraout_dx_00: i32;
    let mut extraout_dx_01: i32;
    let mut extraout_dx_02: i32;
    let mut extraout_dx_03: i32;
    let mut extraout_dx_04: i32;
    let mut extraout_dx_05: i32;
    let mut extraout_dx_06: i32;
    let local_bx_4: *mut Struct734;
    let mut i_var12: i32;
    let local_bx_486: *mut Struct739;
    let local_bx_493: *mut Struct737;
    let mut u_var13: i32;
    let mut u_var14: u16;
    let mut uVar15: u16;
    let mut unaff_cs: u16;
    let mut unaff_ss: u16;
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
    let local_6: *mut Struct735;
    let mut local_4: u16;
    let pu_var11: *mut u8;

    u_var13 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    u_var5 = local_bx_4.field_0x14;
    u_var14 = (u_var5 >> 0x10);
    i_var12 = u_var5;
    _local_6 = u_var5 & 0xffff0000 | (i_var12 + 0xa4);
    if (((i_var12 + 0xa6) != 0) && ((i_var12 + 0xac) != 0)) {
        pass1_1028_081e(param_1);
        u_var9 = local_bx_4.field_0x20;
        pu_var1 = (_local_6 + 8);
        in_dx = extraout_dx;
        let pu_var1_val = unsafe { *pu_var1 };
        if (pu_var1_val < u_var9 || pu_var1_val == u_var9) {
            pu_var7 = local_a;
            ppcVar4 = (param_1 + 0x40);
            (**ppcVar4)();
            in_dx = extraout_dx_00;
            if (pu_var7 == 0x0) {
                if ((_local_6 + 2) == 0xc) {
                    _local_e = pass1_1028_b4f2(param_1);
                    in_dx = (_local_e >> 0x10);
                    local_12 = (_local_e + 0x1f6);
                    plVar2 = (local_12 + 0x170);
                    unsafe { *plVar2 = *plVar2 + 1 };
                } else {
                    local_12 = _PTR_LOOP_1050_68a2;
                    pu_var11 = _PTR_LOOP_1050_68a2;
                    alloc_mem_1000_07fc(_PTR_LOOP_1050_68a2);
                    u_var9 = pu_var11;
                    _local_26 = (pu_var11 & 0xffff | extraout_dx_01 << 0x10);
                    if ((extraout_dx_01 | u_var9) == 0) {
                        local_22 = 0;
                    } else {
                        *_local_26 = ctx.s_1_1050_389a;
                        (u_var9 + 2) = &ctx.PTR_LOOP_1050_1008;
                        (u_var9 + 4) = 0;
                        (u_var9 + 6) = 0;
                        (u_var9 + 8) = 0;
                        (u_var9 + 10) = 0;
                        (u_var9 + 0xc) = 0;
                        *_local_26 = 0x56ce;
                        (u_var9 + 2) = 0x1018;
                        local_22 = _local_26;
                    }
                    u_var14 = (_local_6 >> 0x10);
                    i_var12 = _local_6;
                    uVar15 = (local_22 >> 0x10);
                    (local_22 + 6) = (i_var12 + 2);
                    (local_22 + 10) = (i_var12 + 6);
                    unaff_cs = 0x1020;
                    u_var10 = switch_statement_1020_c3b4((i_var12 + 2));
                    (local_22 + 0xc) = u_var10 * (_local_6 + 6);
                    u_var3 = local_bx_4.field_0x22;
                    ppcVar4 = (local_bx_4.field_0x22 + 4);
                    (**ppcVar4)(0x1020, u_var3, (u_var3 >> 0x10));
                    in_dx = extraout_dx_02;
                }
            }
            local_bx_4.field_0x20 = 0;
        }
    }
    u_var14 = (_local_6 >> 0x10);
    if (((_local_6 + 4) != 0) && ((_local_6 + 8) != 0)) {
        pass1_1028_081e(param_1);
        u_var9 = local_bx_4.field_0x20;
        pu_var1 = (_local_6 + 8);
        in_dx = extraout_dx_03;
        let pu_var1_val = unsafe { *pu_var1 };
        if (pu_var1_val < u_var9 || pu_var1_val == u_var9) {
            puVar8 = &local_2a;
            ppcVar4 = (param_1 + 0x40);
            (**ppcVar4)(unaff_cs, param_1, u_var13);
            in_dx = extraout_dx_04;
            if (puVar8 == 0x0) {
                local_12 = _PTR_LOOP_1050_68a2;
                pu_var11 = _PTR_LOOP_1050_68a2;
                alloc_mem_1000_07fc(_PTR_LOOP_1050_68a2);
                u_var9 = pu_var11;
                _local_26 = (pu_var11 & 0xffff | extraout_dx_05 << 0x10);
                if ((extraout_dx_05 | u_var9) == 0) {
                    local_22 = 0;
                } else {
                    *_local_26 = ctx.s_1_1050_389a;
                    (u_var9 + 2) = &ctx.PTR_LOOP_1050_1008;
                    (u_var9 + 4) = 0;
                    (u_var9 + 6) = 0;
                    (u_var9 + 8) = 0;
                    (u_var9 + 10) = 0;
                    (u_var9 + 0xc) = 0;
                    *_local_26 = 0x56ce;
                    (u_var9 + 2) = 0x1018;
                    local_22 = _local_26;
                }
                u_var14 = (_local_6 >> 0x10);
                local_bx_486 = _local_6;
                uVar15 = (local_22 >> 0x10);
                local_bx_493 = local_22;
                local_bx_493.field_0x8 = local_bx_486.field_0x4;
                local_bx_493.field_0xa = local_bx_486.field_0x6;
                u_var10 = pass1_1020_c42e(local_bx_486.field_0x4);
                (local_22 + 0xc) = u_var10 * (_local_6 + 6);
                u_var3 = local_bx_4.field_0x22;
                ppcVar4 = (local_bx_4.field_0x22 + 4);
                (**ppcVar4)(0x1020, u_var3, (u_var3 >> 0x10));
                in_dx = extraout_dx_06;
            }
            local_bx_4.field_0x20 = 0;
        }
    }
    if (local_bx_4.field_0xc != 0xe) {
        u_var6 = pass1_1028_b58e((param_1 & 0xffff | u_var13 << 0x10));
        _local_2a = CONCAT31(extraout_var, u_var6) & 0xffff | in_dx << 0x10;
        local_22 = (CONCAT31(extraout_var, u_var6) + 0x2e);
        local_1e = (local_22 + 4);
        pass1_1028_68de(
            CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, &local_138)),
            1,
            local_1e,
        );
        pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_138));
    }
    return;
}

pub fn pass1_1028_081e(param_1: *mut Struct44) {
    let pi_var1: *mut i32;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut u_var4: u32;
    let u_var5: u8;
    let local_AX__1: *mut Struct740;
    let mut u_var6: i32;
    let extraout_var: u32;
    let mut in_dx: u16;
    let local_bx_41: *mut Struct741;
    let mut u_var7: u16;
    let mut local_10: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var5 = pass1_1028_b58e(param_1);
    u_var4 = (CONCAT31(extraout_var, u_var5) + 0x2e);
    i_var2 = (u_var4 + 0x18);
    u_var7 = (param_1 >> 0x10);
    local_bx_41 = param_1;
    pi_var1 = &local_bx_41.field_0x20;
    unsafe { *pi_var1 = *pi_var1 + 1 };
    u_var6 = *ctx._PTR_LOOP_1050_65e2;
    u_var3 = (ctx._PTR_LOOP_1050_65e2 + 2);
    if (i_var2 < 0xfa) {
        u_var6 = u_var6 & 1;
    } else {
        if (0x1c1 < i_var2) {
            if (i_var2 < 0x226) {
                return;
            }
            if ((i_var2 < 0x2ee) && (CONCAT22(u_var3, u_var6) % 3 != 0)) {
                return;
            }
            pi_var1 = &local_bx_41.field_0x20;
            unsafe { *pi_var1 = *pi_var1 + 1 };
            return;
        }
        u_var6 = (CONCAT22(u_var3, u_var6) % 3);
    }
    if (u_var6 != 0) {
        return;
    }
    pi_var1 = &local_bx_41.field_0x20;
    unsafe { *pi_var1 = *pi_var1 + -1 };
    return;
}

pub fn pass1_1028_08c6(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1028_0138(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_0954(param_1: *mut Struct742) -> *mut Struct742 {
    let local_bx_12: *mut Struct742;
    let local_es_12: *mut Struct742;

    pass1_1028_b354(param_1);
    local_es_12 = (param_1 >> 0x10);
    local_bx_12 = param_1;
    local_bx_12.field_0x20 = 0;
    param_1 = 0xada;
    local_bx_12.field_0x2 = &PTR_LOOP_1050_1028;
    local_bx_12.field_0xe = 0x4b;
    return param_1;
}

pub fn pass1_1028_0982(param_1: *mut Struct743, param_2: u32, param_3: u32) {
    let mut local_EAX__1: u32;
    let mut local_register2: u16;
    let mut u_var1: u16;

    local_register2 = (local_EAX__1 >> 0x10);
    u_var1 = param_2;
    pass1_1028_b39e(CONCAT22(u_var1, param_1), (param_2 >> 0x10), param_3);
    param_1.field_0x20 = 0;
    CONCAT22(u_var1, param_1) = 0xada;
    param_1.field_0x2 = &PTR_LOOP_1050_1028;
    param_1.field_0xe = 0x4b;
    return CONCAT22(local_register2, param_1);
}

pub fn pass1_1028_09b8(param_1: *mut Struct44) {
    let mut local_DXAX_10: u32;

    local_DXAX_10._0_1_ = pass1_1028_b58e(param_1);
    (CONCAT11(local_DXAX_10._1_1_, local_DXAX_10) + 0x14) = 1;
    return;
}

pub fn pass1_1028_09d4(param_1: u32, param_2: u32, param_3: u32, param_4: u32) {
    let mut u_var1: u32;
    let paVar2: *mut Struct493;
    let pu_var3: *mut u8;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let mut u_var5: u32;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: [u8; 2];
    let mut local_4: u16;

    u_var6 = param_1;
    u_var7 = (param_1 >> 0x10);
    u_var8 = (param_3 >> 0x10);
    local_4 = pass1_1028_c314(u_var6, u_var7, param_2, param_3, u_var8, param_4);
    if (local_4 == 0) {
        return;
    }
    pass1_1028_c7b6(param_1, param_2, param_4);
    if ((local_4 != 0) && (local_4 != 4)) {
        if (((local_4 - 5) < 1) || (SBORROW2(local_4 - 5, 1) || (3 < (local_4 - 6)))) {
            if (((u_var6 + 0xc) != 0x3e) && ((u_var6 + 0xc) != 0x41)) {
                return;
            }
            u_var5 = pass1_1030_bcae(local_6, unaff_ss);
            u_var4 = (u_var5 >> 0x10);
            paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, param_3, u_var8);
            u_var1 = &paVar2.field_0x10;
            paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
            pu_var3 = local_6;
            pass1_1030_bcde(
                pu_var3,
                unaff_ss,
                CONCAT22(u_var4, paVar2),
                param_2,
                param_4,
            );
            if (pu_var3 < 0) {
                PTR_LOOP_1050_50ca = 0x6af;
                return;
            }
            if (5 < pu_var3) {
                PTR_LOOP_1050_50ca = 0x6b5;
                return;
            }
            return;
        }
    }
    PTR_LOOP_1050_50ca = 0x6a8;
    return;
}

pub fn pass1_1028_0ab4(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
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

pub fn pass1_1028_0b64(param_1: *mut Struct744, param_2: u16, param_3: u16, param_3_00: u32) {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    CONCAT22(param_2, param_1) = 0xbbc;
    param_1.field_0x2 = &PTR_LOOP_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_0b96(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_0c24(param_1: *mut Struct745) -> *mut Struct745 {
    let local_bx_14: *mut Struct745;
    let mut u_var1: u16;

    pass1_1028_b354(param_1);
    u_var1 = (param_1 >> 0x10);
    local_bx_14 = param_1;
    local_bx_14.field_0x20 = 0;
    local_bx_14.field_0x22 = 0;
    param_1 = (s_480_bmp_1050_1721 + 3);
    local_bx_14.field_0x2 = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_0c50(param_1: *mut Struct746, param_2: u32, param_3: u32) {
    let mut in_eax: u32;
    let mut u_var1: u16;

    u_var1 = param_2;
    pass1_1028_b39e(CONCAT22(u_var1, param_1), (param_2 >> 0x10), param_3);
    param_1.field_0x20 = 0;
    param_1.field_0x22 = 0;
    CONCAT22(u_var1, param_1) = (s_480_bmp_1050_1721 + 3);
    param_1.field_0x2 = &PTR_LOOP_1050_1028;
    return (ZEXT24(param_1) | in_eax & 0xffff0000);
}

pub fn pass1_1028_0c84(param_1: *mut Struct44, param_2: *mut u8) {
    let pp_var1: fn();
    let u_var2: u8;
    let mut i_var3: i32;
    let pu_var4: *mut u32;
    let pa_var5: *mut Struct493;
    let extraout_var: u32;
    let mut in_dx: i32;
    let mut extraout_dx: u16;
    let mut u_var6: u16;
    let mut unaff_ss: u16;
    let mut u_var7: u32;
    let puVar8: *mut u32;
    let mut u_var9: u32;
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

    u_var2 = pass1_1028_b58e(param_1);
    i_var3 = CONCAT31(extraout_var, u_var2);
    _local_6 = (CONCAT31(extraout_var, u_var2) & 0xffff | in_dx << 0x10);
    local_c = (i_var3 + 0xc);
    local_12 = (i_var3 + 0x10);
    local_1c = &local_c;
    local_e = local_12;
    local_8 = local_12;
    pass1_1028_bab6(param_1);
    local_14 = pass1_1030_2fac(CONCAT22(in_dx, local_12));
    local_1a = local_c;
    local_16 = local_8;
    local_24 = CONCAT22(local_24._2_2_, &local_1a);
    local_e = local_e + 1;
    if (local_e <= local_14) {
        u_var9 = CONCAT22(unaff_ss, local_32);
        local_16 = local_e;
        u_var7 = pass1_1028_bb24(param_1);
        pu_var4 = &local_1a;
        pass1_1030_64ce(
            _PTR_LOOP_1050_5740,
            pu_var4,
            unaff_ss,
            u_var7,
            (u_var7 >> 0x10),
            u_var9,
        );
        unsafe { local_28 = *pu_var4 };
        u_var6 = (pu_var4 + 2);
        local_3a._3_1_ = (local_28 >> 0x18);
        local_24 = local_28;
        if (local_3a._3_1_ != '\0') {
            pa_var5 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_28, u_var6);
            puVar8 = pass1_1030_73a8(CONCAT22(u_var6, pa_var5));
            unsafe {
                pp_var1 = (*puVar8 + 0x58);
                (**pp_var1)(0x1030, puVar8, (puVar8 >> 0x10), param_2);
            }
        }
    }
    pass1_1028_b46e(param_1, param_2);
    pass1_1030_7296(_local_6);
    return;
}

pub fn pass1_1028_0d80(param_1: u32) {
    let mut u_var1: u16;
    let mut u_var2: i32;
    let mut local_4: u16;

    u_var2 = (param_1 >> 0x10);
    u_var1 = (param_1 + 0x20);
    pass1_1028_1646((param_1 & 0xffff | u_var2 << 0x10));
    return u_var1;
}

pub fn pass1_1028_0d9c(param_1: *mut Struct44) {
    let pp_var1: fn();
    let u_var2: u8;
    let local_AX__1: *mut Struct747;
    let pu_var3: *mut u32;
    let paVar4: *mut Struct493;
    let b_var5: bool;
    let extraout_var: u32;
    let extraout_var_00: u32;
    let mut in_dx: u16;
    let mut extraout_dx: u16;
    let mut u_var6: u16;
    let mut unaff_ss: u16;
    let mut u_var7: u32;
    let mut u_var8: u32;
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
    u_var2 = pass1_1028_b58e(param_1);
    local_6 = CONCAT31(extraout_var, u_var2);
    local_c = (local_6 + 0xc);
    local_12 = (local_6 + 0x10);
    local_1c = &local_c;
    local_e = local_12;
    local_8 = local_12;
    pass1_1028_bab6(param_1);
    local_14 = pass1_1030_2fac(CONCAT22(in_dx, local_12));
    local_1a = local_c;
    local_24 = CONCAT22(local_24._2_2_, &local_1a);
    local_16 = local_e + 1;
    if (local_16 <= local_14) {
        u_var8 = CONCAT22(unaff_ss, local_32);
        local_e = local_16;
        u_var7 = pass1_1028_bb24(param_1);
        pu_var3 = &local_1a;
        pass1_1030_64ce(
            _PTR_LOOP_1050_5740,
            pu_var3,
            unaff_ss,
            u_var7,
            (u_var7 >> 0x10),
            u_var8,
        );
        unsafe { local_28 = *pu_var3 };
        u_var6 = (pu_var3 + 2);
        local_3a._3_1_ = (local_28 >> 0x18);
        if (local_3a._3_1_ != '\0') {
            local_24 = local_28;
            paVar4 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_28, u_var6);
            local_3a = CONCAT22(u_var6, paVar4);
            u_var2 = pass1_1030_6fa0(CONCAT22(u_var6, paVar4));
            b_var5 = pass1_1008_c6ae(
                ctx._PTR_LOOP_1050_06e0,
                CONCAT31(extraout_var_00, u_var2),
                0x13,
            );
            if (b_var5 != 0) {
                _local_2c = pass1_1030_73a8(local_3a);
                pp_var1 = (*_local_2c + 0x24);
                (**pp_var1)();
            }
        }
    }
    return;
}

pub fn pass1_1028_0ea6(param_1: u32) {
    let pi_var1: *mut i32;
    let BVar2: bool;
    let local_bx_6: *mut Struct748;
    let mut u_var3: i32;
    let mut local_12: u16;
    let mut local_a: u32;
    let mut local_6: u32;

    u_var3 = (param_1 >> 0x10);
    local_bx_6 = param_1;
    if (local_bx_6.field_0xc != 0x10) {
        BVar2 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, local_bx_6.field_0xc, 0x13);
        if (BVar2 == 0) {
            BVar2 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, local_bx_6.field_0xc, 2);
            if (((BVar2 != 0) && (local_bx_6.field_0x12 != 7)) && (local_bx_6.field_0x12 != 4)) {
                BVar2 = pass1_1028_1556(param_1 & 0xffff | u_var3 << 0x10);
                if (BVar2 == 0) {}
                // goto LAB_1028_0f0a;
                if (local_bx_6.field_0x12 == 9) {
                    local_bx_6.field_0x12 = 5;
                }
            }
        } else {
            if (local_bx_6.field_0x22 < 1) {
                if ((local_bx_6.field_0x12 != 5) && (local_bx_6.field_0x12 != 6)) {
                    return;
                }
                error_check_1000_17ce(local_bx_6.field_0x14);
                local_bx_6.field_0x14 = 0;
                // LAB_1028_0f0a:
                local_bx_6.field_0x12 = 9;
                return;
            }
        }
        pass1_1028_be2a(param_1);
        if (local_bx_6.field_0x12 == 5) {
            unsafe {
                pi_var1 = &local_bx_6.field_0x22;
                *pi_var1 = *pi_var1 + -1
            };
            pass1_1028_b58e((param_1 & 0xffff | u_var3 << 0x10));
        }
    }
    return;
}

pub fn pass1_1028_0fa4(param_1: *mut Struct749) {
    let mut i_var1: i32;
    let BVar2: bool;
    let mut local_AX_105: u16;
    let local_DL_105: u8;
    let mut u_var3: u16;
    let ppVar4: *mut pass1_struct_1;
    let mut local_12: u16;
    let local_10: *mut Struct749;
    let local_e: *mut Struct749;
    let mut local_c: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_10 = param_1;
    local_e = (param_1 >> 0x10);
    pass1_1028_be9e(param_1);
    ppVar4 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_c, 2));
    u_var3 = (ppVar4 >> 0x10);
    i_var1 = (ppVar4 + 0x82);
    if (local_10.field_0x12 == 5) {
        BVar2 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, local_10.field_0xc, 2);
        if ((BVar2 != 0) && (PTR_LOOP_1050_4fbc == 0x0 || (i_var1 != 0))) {
            PTR_LOOP_1050_4fbc = (&ctx.PTR_LOOP_1050_0000 + 1);
            local_AX_105._0_1_ = pass1_1028_b58e(param_1);
            local_AX_105 = CONCAT11(local_AX_105._1_1_, local_AX_105);
            pass1_1030_7c50(
                CONCAT13((u_var3 >> 8), CONCAT12(u_var3, local_AX_105)),
                1,
                4,
            );
        }
        local_10.field_0x22 = 100;
    }
    return;
}

pub fn pass1_1028_1024(param_1: *mut Struct44) -> i32 {
    let u_var1: u8;
    let mut in_AX: u16;
    let extraout_AH: u8;
    let BVar2: bool;
    let paVar3: *mut Struct493;
    let mut in_dx: u16;
    let mut u_var4: i32;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut u_var6: u32;
    let mut u_var7: u32;
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
    local_8 = pass1_1030_2fac(CONCAT22(in_dx, in_AX));
    _local_c = pass1_1028_bb24(param_1);
    local_e = (_local_c >> 0x10);
    u_var1 = pass1_1028_b58e(param_1);
    local_10 = CONCAT11(extraout_AH, u_var1);
    local_16 = (local_10 + 0xc);
    local_1a = 0;
    while (true) {
        if (local_8 < local_1a) {
            return local_1a._2_2_;
        }
        local_12 = local_1a;
        u_var6 = pass1_1030_627e(_PTR_LOOP_1050_5740, CONCAT22(unaff_ss, &local_16), _local_c);
        u_var4 = (u_var6 >> 0x10);
        paVar3 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var6, u_var4);
        if ((u_var4 | paVar3) == 0) {
            break;
        }
        u_var7 = pass1_1030_73a8(CONCAT22(u_var4, paVar3));
        u_var5 = (u_var7 >> 0x10);
        if (u_var7 == 0) {
            return local_1a._2_2_;
        }
        BVar2 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, (u_var7 + 0xc), 0x13);
        if ((BVar2 != 0) && ((u_var7 + 0x12) == 5)) {
            local_1a = local_1a & 0xffff | (local_1a._2_2_ + 1) << 0x10;
        }
        local_1a = local_1a & 0xffff0000 | (local_1a + 1);
    }
    return local_1a._2_2_;
}

pub fn pass1_1028_1106(param_1: *mut Struct44) -> i32 {
    let u_var1: u8;
    let mut in_AX: u16;
    let extraout_AH: u8;
    let BVar2: bool;
    let paVar3: *mut Struct493;
    let mut in_dx: u16;
    let mut u_var4: i32;
    let mut unaff_ss: u16;
    let mut u_var5: u32;
    let mut u_var6: u32;
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
    local_8 = pass1_1030_2fac(CONCAT22(in_dx, in_AX));
    _local_c = pass1_1028_bb24(param_1);
    local_e = (_local_c >> 0x10);
    u_var1 = pass1_1028_b58e(param_1);
    local_10 = CONCAT11(extraout_AH, u_var1);
    local_16 = (local_10 + 0xc);
    local_1a = 0;
    while (true) {
        if (local_8 < local_1a) {
            return local_1a._2_2_;
        }
        local_12 = local_1a;
        u_var5 = pass1_1030_627e(_PTR_LOOP_1050_5740, CONCAT22(unaff_ss, &local_16), _local_c);
        u_var4 = (u_var5 >> 0x10);
        paVar3 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var5, u_var4);
        if ((u_var4 | paVar3) == 0) {
            break;
        }
        u_var6 = pass1_1030_73a8(CONCAT22(u_var4, paVar3));
        if (u_var6 == 0) {
            return local_1a._2_2_;
        }
        BVar2 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, (u_var6 + 0xc), 0x13);
        if (BVar2 != 0) {
            local_1a = local_1a & 0xffff | (local_1a._2_2_ + 1) << 0x10;
        }
        local_1a = local_1a & 0xffff0000 | (local_1a + 1);
    }
    return local_1a._2_2_;
}

pub fn pass1_1028_11de(param_1: *mut Struct44) -> bool {
    let mut local_DXAX_12: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_DXAX_12._0_1_ = pass1_1028_b58e(param_1);
    return (CONCAT11(local_DXAX_12._1_1_, local_DXAX_12) + 0x10) == 0;
}

pub fn pass1_1028_121e(param_1: *mut Struct44) -> *mut Struct44 {
    let u_var1: u8;
    let extraout_AH: u8;
    let paVar2: *mut Struct493;
    let mut u_var3: i32;
    let mut unaff_ss: u16;
    let mut u_var4: u32;
    let mut u_var5: u32;
    let paVar6: *mut Struct44;
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

    u_var4 = pass1_1028_11de(param_1);
    local_4 = (u_var4 >> 0x10);
    if (u_var4 != 0) {
        return param_1;
    }
    u_var1 = pass1_1028_b58e(param_1);
    local_6 = CONCAT11(extraout_AH, u_var1);
    local_c = (local_6 + 0xc);
    local_8 = 0;
    u_var5 = pass1_1028_bb24(param_1);
    u_var4 = pass1_1030_627e(_PTR_LOOP_1050_5740, CONCAT22(unaff_ss, &local_c), u_var5);
    u_var3 = (u_var4 >> 0x10);
    paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var4, u_var3);
    if ((u_var3 | paVar2) == 0) {
        return 0x0;
    }
    paVar6 = pass1_1030_73a8(CONCAT22(u_var3, paVar2));
    return paVar6;
}

pub fn pass1_1028_12be(param_1: u32, param_2: u32) -> bool {
    let pi_var1: *mut i32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let u_var4: u8;
    let b_var5: bool;
    let extraout_AH: u8;
    let mut u_var6: i32;
    let mut i_var7: i32;
    let mut i_var8: i32;
    let mut u_var9: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var9 = pass1_1028_11de(param_1);
    i_var7 = (u_var9 >> 0x10);
    if (u_var9 == 0) {
        _local_c = pass1_1028_121e(param_1);
        ppc_var3 = (*_local_c + 0x40);
        b_var5 = (**ppc_var3)();
        return b_var5;
    }
    param_2 = 0;
    u_var4 = pass1_1028_b58e(param_1);
    local_8 = 4;
    u_var6 = CONCAT11(extraout_AH, u_var4);
    i_var8 = i_var7;
    while {
        pass1_1030_7c28(CONCAT22(i_var7, CONCAT11(extraout_AH, u_var4)), local_8);
        u_var2 = param_2;
        param_2 = param_2 + u_var6;
        pi_var1 = (param_2 + 2);
        unsafe { *pi_var1 = *pi_var1 + i_var8 + CARRY2(u_var2, u_var6) };
        local_8 = local_8 + 1;
        local_8 < 0xe
    } {}
    if (500 < param_2) {
        return 1;
    }
    return 0;
}

pub fn pass1_1028_134a(param_1: *mut u32) {
    let pi_var1: *mut i32;
    let ppc_var2: fn();
    let u_var3: u8;
    let b_var4: bool;
    let pu_var5: *mut u32;
    let mut u_var6: u32;
    let extraout_var: u32;
    let mut in_dx: i32;
    let mut i_var7: i32;
    let mut u_var8: u16;
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

    u_var8 = (param_1 >> 0x10);
    i_var7 = param_1;
    b_var4 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, (i_var7 + 0xc), 0x13);
    if (b_var4 != 0) {
        pu_var5 = &local_6;
        unsafe { ppc_var2 = (*param_1 + 0x40) };
        ppc_var2(&ctx.PTR_LOOP_1050_1008, i_var7, u_var8, pu_var5);
        if (pu_var5 != 0x0) {
            pi_var1 = (i_var7 + 0x22);
            unsafe { *pi_var1 = *pi_var1 + 1 };
            return;
        }
        local_a = 500 - local_6;
        u_var6 = local_a;
        pass1_1028_121e(param_1, u_var8);
        u_var3 = pass1_1028_b58e((u_var6 & 0xffff | in_dx << 0x10));
        local_16 = 0;
        while (local_16 < 10) {
            local_a._0_2_ = (local_16 * 2 + 0x4fbe);
            local_a._2_2_ = local_a >> 0xf;
            if (local_a < local_a) {}
            _local_1a = CONCAT22(local_a._2_2_, local_a);
            pass1_1030_7ddc(
                CONCAT31(extraout_var, u_var3) & 0xffff | in_dx << 0x10,
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
    let mut i_var1: i32;
    let mut u_var2: u32;

    i_var1 = pass1_1028_11de(param_1);
    if (i_var1 == 0) {
        u_var2 = pass1_1028_121e(param_1);
        i_var1 = pass1_1028_1416(u_var2);
        return i_var1;
    }
    i_var1 = pass1_1028_1024(param_1);
    return i_var1 * 0xf;
}

pub fn pass1_1020_e9d4(param_1: u16, param_2: u16, param_3: u16) {
    let u_var1: u8;
    let extraout_var: u32;
    let mut extraout_dx: u16;
    let mut u_var2: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1030_df0c(param_1, param_2, param_3);
    u_var2 = extraout_dx;
    u_var1 = pass1_1028_b58e(CONCAT22(param_2, param_1));
    pass1_1038_57dc(
        *(CONCAT31(extraout_var, u_var1) + 0x2e),
        0x1,
        (&ctx.PTR_LOOP_1050_0000 + 1),
    );
    return;
}

pub fn pass1_1020_ea0e(param_1: u32) {
    pass1_1028_bdac(param_1, 1);
    return;
}

pub fn pass1_1020_ea20(param_1: u32, param_2: u32, param_3: u32, param_4: u32) {
    let pp_var1: fn();
    let mut u_var2: u16;
    let mut cVar3: u8;
    let mut in_AX: i32;
    let pu_var4: *mut u32;
    let pa_var5: *mut Struct493;
    let mut u_var6: u16;
    let mut extraout_dx: u16;
    let mut u_var7: i32;
    let mut u_var8: u16;
    let mut unaff_ss: u16;
    let mut u_var9: u16;
    let mut u_var10: i32;
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

    u_var9 = (param_1 >> 0x10);
    u_var2 = param_1;
    pass1_1028_c3aa(u_var2, u_var9, param_2, param_3, param_4);
    if (in_AX == 0) {
        return;
    }
    pass1_1028_c23e(u_var2, u_var9, param_2, param_3, param_4);
    if (in_AX == 0) {
        return;
    }
    local_4 = pass1_1028_c314(u_var2, u_var9, param_2, param_3, (param_3 >> 0x10), param_4);
    if (local_4 == 0) {
        return;
    }
    u_var8 = (param_2 >> 0x10);
    pass1_1028_c7b6(param_1, u_var9, param_2, u_var8, param_4);
    if ((((local_4 == 5) || (local_4 == 4)) || (local_4 == 6)) || (local_4 == 9)) {
        PTR_LOOP_1050_50ca = 0x6a8;
        return;
    }
    if (local_4 != 0) {
        return;
    }
    pu_var4 = &local_12;
    pass1_1030_64ce(
        _PTR_LOOP_1050_5740,
        (_PTR_LOOP_1050_5740 >> 0x10),
        param_2,
        u_var8,
        param_4,
        (param_4 >> 0x10),
        pu_var4,
        unaff_ss,
    );
    unsafe { local_26 = *pu_var4 };
    local_38 = (pu_var4 + 2);
    local_26._3_1_ = (local_26 >> 0x18);
    local_3a = local_26._3_1_;
    local_1c = local_26;
    local_8 = local_26;
    if (local_26._3_1_ == 0) {}
    // goto LAB_1020_eb4e;
    local_8._0_2_ = local_26;
    pa_var5 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_8, local_38);
    local_26 = CONCAT22(local_38, pa_var5);
    local_22 = &pa_var5[1].field_0x10;
    if ((local_22 + 4) != param_3) {
        PTR_LOOP_1050_50ca = 0x6b7;
        return;
    }
    local_1c = pass1_1030_73a8(CONCAT22(local_38, pa_var5));
    local_38 = (local_1c >> 0x10);
    u_var7 = (local_1c + 0xc);
    local_3a = u_var7;
    if (u_var7 != 0x41) {
        if (0x41 < u_var7) {
            if (u_var7 == 0x6b) {
                PTR_LOOP_1050_50ca = 0x6b1;
                return;
            }
            if (u_var7 < 0x6c) {
                if (u_var7 == 0x42) {
                    PTR_LOOP_1050_50ca = 0x6b1;
                    return;
                }
                local_3a = u_var7 - 0x4b;
                if (local_3a == 0) {
                    PTR_LOOP_1050_50ca = 0x6b1;
                    return;
                }
            } else {
                if (u_var7 == 0x6e) {
                    return;
                }
                local_3a = u_var7 - 0x73;
                if ((4 < (u_var7 - 0x6e))
                    && (
                        local_3a = u_var7 - 0x79,
                        local_3a == 0 || (u_var7 - 0x73) < 6,
                    ))
                {
                    PTR_LOOP_1050_50ca = 0x6b0;
                    return;
                }
            }
            // goto LAB_1020_eb4e;
        }
        if (u_var7 != 0x3e) {
            if (u_var7 < 0x3f) {
                cVar3 = u_var7;
                if (cVar3 != 0xb) {
                    if (cVar3 == 0x10) {
                        return;
                    }
                    local_3a = u_var7 & 0xff00 | (cVar3 - 0x37);
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
    u_var8 = 0x1000;
    process_struct_1000_179c(0xb4, local_38);
    u_var7 = local_38 | local_3a;
    if (u_var7 == 0) {
        local_e = 0;
        u_var7 = 0;
    } else {
        u_var8 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
        local_e = local_3a;
        mixed_1040_8520(
            CONCAT13((local_38 >> 8), CONCAT12(local_38, local_3a)),
            ctx.g_h_window,
            0x24,
            2,
            0x57b,
            0x5e8,
        );
    }
    _local_c = CONCAT22(u_var7, local_e);
    pp_var1 = (*_local_c + 0x74);
    u_var10 = local_e;
    (**pp_var1)(u_var8, local_e, u_var7);
    if (local_e != 7) {
        local_2e = local_8;
        local_22 = local_8;
        local_22._3_1_ = (local_8 >> 0x18);
        if (local_22._3_1_ != '\0') {
            pa_var5 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_8, local_8._2_2_);
            _local_32 = CONCAT22(local_8._2_2_, pa_var5);
            pass1_1030_7296(CONCAT22(local_8._2_2_, pa_var5));
            pass1_1030_730a(_local_32);
            local_2e =
                process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(u_var10, 0x2f));
            pass1_1010_ec68(local_2e, _local_32);
            _local_2a = pass1_1030_73a8(_local_32);
            pp_var1 = (*_local_2a + 0x24);
            (**pp_var1)(0x1030, _local_2a, (_local_2a >> 0x10));
            u_var6 = pass1_1028_bc4a(_local_2a);
            (u_var2 + 0x24) = u_var6;
            pass1_1030_e4fa(CONCAT22(unaff_ss, &local_146), (_local_32 + 0x16));
            pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_146));
        }
        return;
    }
    PTR_LOOP_1050_50ca = (&ctx.PTR_LOOP_1050_0000 + 1);
    return;
}

pub fn pass1_1020_ecb0(param_1: u32) {
    let mut u_var1: u32;
    let paVar2: *mut Struct493;
    let mut in_dx: u16;
    let mut i_var3: i32;
    let mut u_var4: i32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    u_var1 = (i_var3 + 8);
    paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
    if ((i_var3 + 0x12) == 1) {
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
        (i_var3 + 0x14) = local_8;
        return;
    }
    pass1_1028_bf22((param_1 & 0xffff | u_var4 << 0x10));
    return;
}

pub fn pass1_1020_ed3c(param_1: *mut Struct44) {
    let pi_var1: *mut i32;
    let u_var2: u8;
    let extraout_var: u32;
    let extraout_var_00: u32;
    let mut in_dx: u16;
    let mut extraout_dx: i32;
    let mut u_var3: i32;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
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

    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    pi_var1 = (i_var4 + 0x14);
    unsafe { *pi_var1 = *pi_var1 + -1 };
    let pi_var1_val = unsafe { *pi_var1 };
    if (pi_var1_val == 0) {
        (i_var4 + 0x12) = 0;
        u_var2 = pass1_1028_b58e(param_1);
        local_6 = CONCAT31(extraout_var, u_var2);
        local_c = (local_6 + 0xc);
        uStack8 = (local_6 + 0x10);
        local_1e = &local_c;
        local_4 = in_dx;
        pass1_1008_3eb4(
            CONCAT22(unaff_ss, &local_c),
            CONCAT22(unaff_ss, &local_12),
            CONCAT22(unaff_ss, local_10),
            CONCAT22(unaff_ss, local_e),
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
        u_var3 = extraout_dx;
        u_var2 = pass1_1028_b58e(param_1);
        _local_18 = CONCAT31(extraout_var_00, u_var2) & 0xffff | u_var3 << 0x10;
        local_1c = *(CONCAT31(extraout_var_00, u_var2) + 0x2e);
        pass1_1038_5804(local_1c, 1, (&ctx.PTR_LOOP_1050_0000 + 1));
        if (0 < (i_var4 + 0x24)) {
            local_26 = (local_1c + 4);
            pass1_1028_68de(CONCAT22(unaff_ss, &local_138), (i_var4 + 0x24), local_26);
            pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_138));
        }
    }
    return;
}

pub fn pass1_1020_ee3a(param_1: u32, param_2: u16) {
    let u_var1: u8;
    let extraout_var: u32;
    let mut in_dx: u16;
    let mut unaff_ss: u16;
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

    u_var1 = pass1_1028_b58e(param_1);
    local_6 = CONCAT31(extraout_var, u_var1);
    local_c = (local_6 + 0xc);
    uStack8 = (local_6 + 0x10);
    local_4 = in_dx;
    _local_10 = pass1_1028_bb24(param_1);
    local_14 = (local_6 + 0x2e);
    local_18 = (local_14 + 4);
    pass1_1028_87f0(
        CONCAT22(unaff_ss, &local_13c),
        0,
        1,
        param_2,
        &local_c,
        unaff_ss,
        local_18,
        _local_10,
    );
    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_13c));
    return;
}

pub fn pass1_1020_eed0(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1030_dcf4(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_ef5e(param_1: *mut Struct44) {
    param_1.ptr_a_lo = 0;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    pass1_1028_b418(param_1);
    return;
}

pub fn pass1_1020_ef94(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1020_ef5e(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_0068(param_1: *mut Struct727) {
    let mut u_var1: i32;
    let paVar2: *mut Struct199;
    let struct_a: *mut Struct199;
    let mut u_var3: i32;
    let local_bx_13: *mut Struct727;
    let local_es_13: *mut Struct727;
    let mut local_4: u16;

    pass1_1028_b354(param_1);
    local_es_13 = (param_1 >> 0x10);
    local_bx_13 = param_1;
    u_var1 = 0;
    local_bx_13.field_0x20 = 0;
    &local_bx_13.field_0x22 = 0;
    param_1 = 0x8ec;
    local_bx_13.field_0x2 = &PTR_LOOP_1050_1028;
    paVar2 = struct_a;
    process_struct_1000_179c(0xc, struct_a);
    u_var3 = paVar2 | u_var1;
    if (u_var3 == 0) {
        &local_bx_13.field_0x22 = 0;
    } else {
        paVar2 = process_struct_1008_574a(CONCAT22(paVar2, u_var1));
        local_bx_13.field_0x22 = paVar2;
        local_bx_13.field_0x24 = u_var3;
    }
    return;
}

pub fn pass1_1028_00cc(param_1: *mut Struct728, param_2: u32, param_3: u32) {
    let mut u_var1: i32;
    let paVar2: *mut Struct199;
    let struct_a: *mut Struct199;
    let mut u_var3: i32;
    let mut u_var4: u16;
    let mut local_4: u16;

    u_var4 = param_2;
    pass1_1028_b39e(CONCAT22(u_var4, param_1), (param_2 >> 0x10), param_3);
    u_var1 = 0;
    param_1.field_0x20 = 0;
    &param_1.field_0x22 = 0;
    CONCAT22(u_var4, param_1) = 0x8ec;
    param_1.field_0x2 = &PTR_LOOP_1050_1028;
    paVar2 = struct_a;
    process_struct_1000_179c(0xc, struct_a);
    u_var3 = paVar2 | u_var1;
    if (u_var3 == 0) {
        &param_1.field_0x22 = 0;
    } else {
        paVar2 = process_struct_1008_574a(CONCAT22(paVar2, u_var1));
        param_1.field_0x22 = paVar2;
        param_1.field_0x24 = u_var3;
    }
    return;
}

pub fn pass1_1028_0138(param_1: *mut Struct44) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let mut i_var4: i32;
    let mut u_var5: u16;

    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    param_1.ptr_a_lo = 0x8ec;
    (i_var4 + 2) = &PTR_LOOP_1050_1028;
    pu_var1 = (i_var4 + 0x22);
    u_var2 = (i_var4 + 0x24);
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            ppc_var3 = *pu_var1;
            (**ppc_var3)()
        };
    }
    pass1_1028_b418(param_1);
    return;
}

pub fn pass1_1028_0176(param_1: *mut Struct44, param_2: *mut u8) {
    let pu_var1: *mut u32;
    let ppc_var2: fn();
    let u_var3: u8;
    let paVar4: *mut Struct199;
    let mut u_var5: i32;
    let mut u_var6: i32;
    let pa_var7: *mut Struct199;
    let mut i_var8: i32;
    let mut u_var9: u16;
    let mut local_c: u16;

    i_var8 = param_1;
    u_var9 = (param_1 >> 0x10);
    pass1_1028_b46e(param_1, param_2);
    pu_var1 = (i_var8 + 0x22);
    u_var6 = (i_var8 + 0x24);
    u_var5 = u_var6 | pu_var1;
    pa_var7 = CONCAT22(u_var5, pu_var1);
    if (u_var5 != 0) {
        unsafe {
            ppc_var2 = *pu_var1;
            pa_var7 = ppc_var2()
        };
    }
    process_struct_1000_179c(0xc, (pa_var7 >> 0x10));
    u_var6 = (pa_var7 >> 0x10) | pa_var7;
    if (pa_var7 == 0x0) {
        paVar4 = 0x0;
        u_var6 = 0;
    } else {
        paVar4 = process_struct_1008_574a(pa_var7);
    }
    (i_var8 + 0x22) = paVar4;
    (i_var8 + 0x24) = u_var6;
    u_var9 = 0x14;
    u_var3 = pass1_1028_b58e(param_1);
    pass1_1030_7f1a(u_var3, u_var6, u_var9);
    return;
}

pub fn pass1_1028_01ec(param_1: *mut Struct729) {
    let mut u_var1: u32;
    let local_bx_4: *mut Struct729;
    let local_bx_21: *mut Struct730;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    if ((local_bx_4.field_0x12 == 6) || (local_bx_4.field_0x12 == 5)) {
        u_var1 = local_bx_4.field_0x14;
        u_var2 = (u_var1 >> 0x10);
        local_bx_21 = u_var1;
        if ((local_bx_21.field_0xa6 == 0x14) || (local_bx_21.field_0xa8 == 0x10)) {
            pass1_1028_bdac(param_1, 6);
            return;
        }
        pass1_1028_be2a(param_1);
    }
    return;
}

pub fn pass1_1020_e76c(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
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

pub fn pass1_1020_e846(param_1: *mut Struct44) {
    param_1.ptr_a_lo = 0xe88e;
    (param_1 + 2) = 0x1020;
    pass1_1028_b418(param_1);
    return;
}

pub fn pass1_1020_e868(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1020_e846(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_e8f6(param_1: *mut u16) {
    let mut u_var1: u16;

    pass1_1030_dc96(param_1);
    u_var1 = (param_1 >> 0x10);
    (param_1 + 0x24) = 0;
    unsafe {
        *param_1 = 0xeef6;
        (param_1 + 2) = 0x1020
    };
    return param_1;
}

pub fn pass1_1020_e91e(param_1: *mut Struct1012, param_2: u16, param_3: u16, param_3_00: u32) {
    pass1_1030_dcc2(param_1, param_2, param_3, param_3_00);
    (param_1 + 1) = 0;
    CONCAT22(param_2, param_1) = 0xeef6;
    param_1.field_0x2 = 0x1020;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1020_d4ca(param_1: *mut Struct44) {
    let u_var1: u8;
    let mut u_var2: u16;
    let extraout_var: u32;
    let pu_var3: *mut u8;
    let mut in_dx: u16;
    let mut extraout_dx: i32;
    let mut i_var4: i32;
    let mut local_6: u16;
    let mut local_4: u16;

    if ((param_1 + 0x20) == 2) {
        return;
    }
    u_var1 = pass1_1028_b58e(param_1);
    pu_var3 = *(CONCAT31(extraout_var, u_var1) + 0x2e);
    i_var4 = 99;
    pass1_1038_3fb0(pu_var3);
    u_var2 = pass1_1030_25b2(pu_var3 & 0xffff | extraout_dx << 0x10, i_var4);
    if (u_var2 != 0) {
        return;
    }
    return;
}

pub fn pass1_1020_d518(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
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
    let pp_var1: fn();
    let u_var2: u8;
    let pu_var3: *mut u32;
    let paVar4: *mut Struct493;
    let extraout_var: u32;
    let mut in_dx: u16;
    let mut extraout_dx: u16;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut u_var6: u32;
    let pu_var7: *mut u32;
    let mut u_var8: u32;
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

    u_var2 = pass1_1028_b58e(param_1);
    local_6 = CONCAT31(extraout_var, u_var2);
    local_c = (local_6 + 0xc);
    local_12 = (local_6 + 0x10);
    local_1c = &local_c;
    local_e = local_12;
    local_8 = local_12;
    pass1_1028_bab6(param_1);
    local_14 = pass1_1030_2fac(CONCAT22(in_dx, local_12));
    local_1a = local_c;
    local_16 = local_8;
    local_24 = CONCAT22(local_24._2_2_, &local_1a);
    local_e = local_e + 1;
    if (local_e < local_14) {
        u_var8 = CONCAT22(unaff_ss, local_32);
        local_16 = local_e;
        u_var6 = pass1_1028_bb24(param_1);
        pu_var3 = &local_1a;
        pass1_1030_64ce(
            _PTR_LOOP_1050_5740,
            pu_var3,
            unaff_ss,
            u_var6,
            (u_var6 >> 0x10),
            u_var8,
        );
        unsafe { local_28 = *pu_var3 };
        u_var5 = (pu_var3 + 2);
        local_3a._3_1_ = (local_28 >> 0x18);
        local_24 = local_28;
        if (local_3a._3_1_ != '\0') {
            paVar4 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_28, u_var5);
            pu_var7 = pass1_1030_73a8(CONCAT22(u_var5, paVar4));
            unsafe { pp_var1 = (*pu_var7 + 0x58) };
            (**pp_var1)(0x1030, pu_var7, (pu_var7 >> 0x10), param_2);
        }
    }
    pass1_1028_b46e(param_1, param_2);
    return;
}

pub fn pass1_1020_d6e6(param_1: *mut Struct44) {
    let pp_var1: fn();
    let u_var2: u8;
    let pu_var3: *mut u32;
    let paVar4: *mut Struct493;
    let extraout_var: u32;
    let mut in_dx: u16;
    let mut extraout_dx: u16;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut u_var6: u32;
    let pu_var7: *mut u32;
    let mut u_var8: u32;
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
    u_var2 = pass1_1028_b58e(param_1);
    local_6 = CONCAT31(extraout_var, u_var2);
    local_c = (local_6 + 0xc);
    local_12 = (local_6 + 0x10);
    local_1c = &local_c;
    local_e = local_12;
    local_8 = local_12;
    pass1_1028_bab6(param_1);
    local_14 = pass1_1030_2fac(CONCAT22(in_dx, local_12));
    local_1a = local_c;
    local_24 = CONCAT22(local_24._2_2_, &local_1a);
    local_16 = local_e + 1;
    if (local_16 < local_14) {
        u_var8 = CONCAT22(unaff_ss, local_32);
        local_e = local_16;
        u_var6 = pass1_1028_bb24(param_1);
        pu_var3 = &local_1a;
        pass1_1030_64ce(
            _PTR_LOOP_1050_5740,
            pu_var3,
            unaff_ss,
            u_var6,
            (u_var6 >> 0x10),
            u_var8,
        );
        unsafe { local_28 = *pu_var3 };
        u_var5 = (pu_var3 + 2);
        local_3a._3_1_ = (local_28 >> 0x18);
        if (local_3a._3_1_ != '\0') {
            local_24 = local_28;
            paVar4 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_28, u_var5);
            pu_var7 = pass1_1030_73a8(CONCAT22(u_var5, paVar4));
            if ((pu_var7 + 0xc) == 0x6a) {
                unsafe {
                    pp_var1 = (*pu_var7 + 0x24);
                    (**pp_var1)()
                };
            }
        }
    }
    return;
}

pub fn pass1_1020_d7d8(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
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

pub fn pass1_1020_d8c6(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_d954(param_1: *mut Struct723) {
    let mut unaff_BP: u16;
    let ppVar1: *mut pass1_struct_1;
    let local_6: *mut Struct723;
    let local_4: *mut Struct723;

    local_6 = param_1;
    local_4 = (param_1 >> 0x10);
    pass1_1030_dc96(param_1);
    local_6.field_0x24 = 0;
    local_6.field_0x26 = 0;
    &local_6.field_0x28 = 0;
    param_1.field_0x0 = 0xe792;
    local_6.field_0x2 = 0x1020;
    ppVar1 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_BP, 0x2f));
    local_6.field_0x28 = ppVar1;
    local_6.field_0x2a = (ppVar1 >> 0x10);
    return;
}

pub fn pass1_1020_d99e(
    param_1: *mut Struct724,
    param_2: u16,
    param_3: u16,
    param_4: u32,
) -> *mut Struct724 {
    let local_bx_22: *mut Struct724;
    let mut unaff_BP: u16;
    let ppVar1: *mut pass1_struct_1;
    let paVar2: *mut Struct1012;
    let mut u_var3: u16;

    paVar2 = param_1;
    u_var3 = (param_1 >> 0x10);
    pass1_1030_dcc2(paVar2, u_var3, param_3, param_4);
    (paVar2 + 1) = param_2;
    paVar2[1].field_0x2 = 0;
    &paVar2[1].field_0x4 = 0;
    param_1 = 0xe792;
    paVar2.field_0x2 = 0x1020;
    ppVar1 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_BP, 0x2f));
    &paVar2[1].field_0x4 = ppVar1;
    &paVar2[1].field_0x6 = (ppVar1 >> 0x10);
    &paVar2.field_0x10 = 0x49;
    return param_1;
}

pub fn pass1_1020_d9fa(param_1: *mut u8, param_2: u16) {
    let u_var1: u8;
    let extraout_var: u32;
    let mut extraout_dx: u16;
    let mut u_var2: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if ((param_1 + 0xc) != 0x79) {
        pass1_1030_df0c(param_1, param_2);
        u_var2 = extraout_dx;
        u_var1 = pass1_1028_b58e(param_1);
        pass1_1038_57dc(
            *(CONCAT31(extraout_var, u_var1) + 0x2e),
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
    let pp_var1: fn();
    let pu_var2: *mut u32;
    let paVar3: *mut Struct493;
    let mut u_var4: u16;
    let mut extraout_dx: u16;
    let mut u_var5: i32;
    let mut u_var6: i32;
    let mut unaff_ss: u16;
    let mut u_var7: u32;
    let mut u_var8: u32;
    let mut u_var9: u32;
    let mut u_var10: u32;
    let mut local_22: u32;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_18: u32;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    pu_var2 = &local_e;
    pass1_1030_64ce(_PTR_LOOP_1050_5740, param_2, param_4, pu_var2, unaff_ss);
    unsafe { local_6 = *pu_var2 };
    u_var4 = (pu_var2 + 2);
    local_22._3_1_ = (local_6 >> 0x18);
    u_var6 = local_22._3_1_;
    if (local_22._3_1_ != 0) {
        paVar3 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_6, u_var4);
        u_var7 = pass1_1030_73a8(CONCAT22(u_var4, paVar3));
        u_var6 = u_var7;
        if ((u_var6 + 0xc) == 0x10) {
            PTR_LOOP_1050_50ca = 0x6a9;
            return;
        }
    }
    pass1_1028_c7b6(param_1, param_2, param_4);
    u_var5 = (param_1 >> 0x10);
    u_var8 = param_1 & 0xffff | u_var5 << 0x10;
    unsafe { pp_var1 = (*param_1 + 0x60) };
    u_var7 = param_2;
    u_var9 = param_3;
    u_var10 = param_4;
    local_8 = u_var6;
    (**pp_var1)();
    if (((u_var6 != 0)
        && (
            pass1_1028_c23e(param_1, u_var5, param_2, param_3, param_4),
            u_var6 != 0,
        ))
        && (
            u_var4 = pass1_1028_c314(
                param_1,
                u_var5,
                param_2,
                param_3,
                (param_3 >> 0x10),
                param_4,
            ),
            u_var4 != 0,
        ))
    {
        u_var6 = (param_2 >> 0x10);
        if ((((param_2 + 4) == 0) && (local_8 != 4))
            && (
                unsafe { pp_var1 = (*param_1 + 0x5c) },
                (**pp_var1)(
                    &PTR_LOOP_1050_1028,
                    param_1,
                    param_2,
                    param_3,
                    param_4,
                    u_var8,
                    u_var7,
                    u_var9,
                    u_var10,
                ),
                u_var4 == 0,
            ))
        {
            return;
        }
        local_a = (param_2 + 4);
        if (local_a != 0) {
            win_fn_1020_df10(param_1, param_2 & 0xffff | u_var6 << 0x10, param_4);
            return;
        }
        pass1_1020_deac(param_1, param_2 & 0xffff | u_var6 << 0x10, param_4);
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
    let paVar2: *mut Struct493;
    let pu_var3: *mut u8;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let mut u_var5: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: [u8; 2];

    u_var5 = pass1_1030_bcae(local_4, unaff_ss);
    u_var4 = (u_var5 >> 0x10);
    paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, param_2_00, (param_2_00 >> 0x10));
    u_var1 = &paVar2.field_0x10;
    u_var5 = param_1_00;
    paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
    pu_var3 = local_4;
    pass1_1030_bcde(pu_var3, unaff_ss, CONCAT22(u_var4, paVar2), u_var5, param_5);
    if (pu_var3 < 0) {
        PTR_LOOP_1050_50ca = 0x6af;
    } else {
        if ((pu_var3 < 0x1f) || ((param_1_00 + 4) < 1)) {
            return;
        }
        PTR_LOOP_1050_50ca = 0x6b6;
        PTR_LOOP_1050_50cc = pu_var3 + -0x1e;
    }
    return;
}

pub fn pass1_1020_dc1c(param_1: u32, param_2: u32) {
    let mut i_var1: i32;
    let ppc_var2: fn();
    let pu_var3: *mut u32;
    let paVar4: *mut Struct493;
    let mut extraout_dx: u16;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut u_var6: u32;
    let pu_var7: *mut u32;
    let mut u_var8: u32;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_14: u32;
    let mut local_a: [u8; 4];
    let mut local_6: u32;

    u_var8 = CONCAT22(unaff_ss, local_a);
    u_var6 = pass1_1028_bb24(param_1);
    pu_var3 = u_var6;
    pass1_1030_64ce(
        _PTR_LOOP_1050_5740,
        param_2,
        pu_var3,
        (u_var6 >> 0x10),
        u_var8,
    );
    unsafe { local_6 = *pu_var3 };
    u_var5 = (pu_var3 + 2);
    local_1e._3_1_ = (local_6 >> 0x18);
    if (local_1e._3_1_ != '\0') {
        paVar4 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_6, u_var5);
        pu_var7 = pass1_1030_73a8(CONCAT22(u_var5, paVar4));
        i_var1 = (pu_var7 + 0xc);
        if (((i_var1 < 1) || (SBORROW2(i_var1, 1)))
            || (i_var1 != 9 && 7 < i_var1 + -1 && (i_var1 + -9 < 0x6a || (6 < i_var1 + -0x73))))
        {
            unsafe {
                ppc_var2 = (*pu_var7 + 0x24);
                ppc_var2()
            };
        }
    }
    return;
}

pub fn pass1_1020_dca8(param_1: *mut Struct44) {
    let mut u_var1: u16;
    let mut u_var2: u32;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let mut extraout_dx: u16;
    let mut u_var7: u16;
    let mut u_var8: u16;
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
    local_DXAX_29._2_2_ = extraout_dx;
    local_DXAX_29._0_1_ = pass1_1028_b58e(param_1);
    u_var2 = (CONCAT11(local_DXAX_29._1_1_, local_DXAX_29) + 0xc);
    u_var1 = (CONCAT11(local_DXAX_29._1_1_, local_DXAX_29) + 0x10);
    local_10 = u_var2;
    u_var3 = (u_var2 >> 0x10);
    u_var4 = local_10 - 1;
    if (u_var4 < 0) {
        u_var4 = 0;
    }
    u_var7 = local_4 - 1;
    u_var5 = local_10 + 1;
    if (u_var7 < (local_10 + 1)) {
        u_var5 = u_var7;
    }
    u_var6 = u_var3 - 1;
    if (u_var6 < 0) {
        u_var6 = 0;
    }
    u_var8 = u_var3 + 1;
    if (u_var7 < (u_var3 + 1)) {
        u_var8 = u_var7;
    }
    pass1_1008_3e54(CONCAT22(local_SS__1, local_2e), u_var1, u_var6, u_var4);
    pass1_1020_dc1c(param_1, CONCAT22(local_SS__1, local_2e));
    pass1_1008_3e54(CONCAT22(local_SS__1, local_2e), u_var1, u_var6, local_10);
    pass1_1020_dc1c(param_1, CONCAT22(local_SS__1, local_2e));
    pass1_1008_3e54(CONCAT22(local_SS__1, local_2e), u_var1, u_var6, u_var5);
    pass1_1020_dc1c(param_1, CONCAT22(local_SS__1, local_2e));
    pass1_1008_3e54(CONCAT22(local_SS__1, local_2e), u_var1, u_var3, u_var4);
    pass1_1020_dc1c(param_1, CONCAT22(local_SS__1, local_2e));
    pass1_1008_3e54(CONCAT22(local_SS__1, local_2e), u_var1, u_var3, u_var5);
    pass1_1020_dc1c(param_1, CONCAT22(local_SS__1, local_2e));
    pass1_1008_3e54(CONCAT22(local_SS__1, local_2e), u_var1, u_var8, u_var4);
    pass1_1020_dc1c(param_1, CONCAT22(local_SS__1, local_2e));
    pass1_1008_3e54(CONCAT22(local_SS__1, local_2e), u_var1, u_var8, local_10);
    pass1_1020_dc1c(param_1, CONCAT22(local_SS__1, local_2e));
    pass1_1008_3e54(CONCAT22(local_SS__1, local_2e), u_var1, u_var8, u_var5);
    pass1_1020_dc1c(param_1, CONCAT22(local_SS__1, local_2e));
    return;
}

pub fn win_fn_1020_de32(param_1: u32, param_2: u16) {
    let mut i_var1: i32;
    let mut u_var2: u16;
    let mut u_var3: u16;
    let ppVar4: *mut pass1_struct_1;
    let mut in_stack_0000ffee: u32;
    let in_string_1: *mut libc::c_char;
    let mut local_6: u16;
    let mut local_4: u16;

    in_string_1 = CONCAT22((in_stack_0000ffee >> 0x10), 5);
    ppVar4 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, in_string_1);
    u_var2 = (ppVar4 >> 0x10);
    (ppVar4 + 0x12) = param_2;
    u_var3 = (in_string_1 >> 0x10);
    i_var1 = gui_window_func_1038_b72e(ctx._g_Struct112_a, (ctx._g_Struct112_a >> 0x10), 4);
    if (i_var1 == 0) {
        pass1_1038_af40(ctx._g_Struct112_a, *(_PTR_LOOP_1050_4230 + 0x16), 4);
    }
    PTR_LOOP_1050_5b80 = (&ctx.PTR_LOOP_1050_0000 + 1);
    process_win_msg_1008_9510(&PTR_LOOP_1050_5b80, &ctx.g_alloc_addr_1050_1050, u_var3);
    u_var3 = (param_1 >> 0x10);
    (param_1 + 0x24) = (ppVar4 + 10);
    if ((param_1 + 0x24) == 0) {
        PTR_LOOP_1050_50ca = 0x6b2;
    }
    return;
}

pub fn pass1_1020_deac(param_1: u32, param_2: u32, param_3: u32) -> bool {
    let mut i_var1: i32;
    let mut u_var2: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    i_var1 = pass1_1028_c7b6(param_1, param_2, param_3);
    if (i_var1 < 1) {
        return 0;
    }
    if (SBORROW2(i_var1, 1)) {
        return 0;
    }
    u_var2 = (param_1 >> 0x10);
    if (i_var1 != 3 && 0 < i_var1 + -2) {
        if (i_var1 == 4) {
            win_fn_1020_de32(param_1, 4);
            if ((param_1 + 0x24) == 6) {
                return 1;
            }
            return 0;
        }
        if (i_var1 != 5) {
            return 0;
        }
    }
    (param_1 + 0x24) = 1;
    return 1;
}

pub fn win_fn_1020_df10(param_1: u32, param_2: u32, param_3: u32) {
    let mut in_AX: u16;
    let pu_var1: *mut u32;
    let paVar2: *mut Struct493;
    let mut u_var3: u16;
    let mut extraout_dx: u16;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let mut u_var5: u32;
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
    u_var4 = (param_1 >> 0x10);
    if (in_AX == 0) {
        pu_var1 = &local_e;
        pass1_1030_64ce(_PTR_LOOP_1050_5740, param_2, param_3, pu_var1, unaff_ss);
        unsafe { local_a = *pu_var1 };
        u_var3 = (pu_var1 + 2);
        local_22._3_1_ = (local_a >> 0x18);
        if (local_22._3_1_ != '\0') {
            paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_a, u_var3);
            u_var5 = pass1_1030_73a8(CONCAT22(u_var3, paVar2));
            if ((u_var5 + 0xc) == 0x6a) {
                u_var3 = pass1_1020_e044(param_1);
                if (u_var3 == 0) {
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
                    u_var3 = pass1_1020_e044(param_1);
                    if (u_var3 != 0) {
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
    let paVar2: *mut Struct493;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let mut u_var5: u32;
    let mut local_e: u16;
    let mut local_c: u16;

    u_var4 = (param_1 >> 0x10);
    u_var5 = pass1_1018_04b8((param_1 + 0x28));
    u_var3 = (u_var5 >> 0x10);
    paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var5, u_var3);
    u_var3 = pass1_1030_2fac(CONCAT22(u_var3, paVar2));
    u_var1 = (param_1 + 0x28);
    if (u_var3 <= (u_var1 + 0x1e)) {
        return 1;
    }
    return 0;
}

pub fn pass1_1020_e08e(param_1: *mut Struct44) {
    let mut i_var1: i32;
    let mut u_var2: u16;
    let u_var3: u8;
    let mut i_var4: i32;
    let pa_var5: *mut Struct493;
    let extraout_var: u32;
    let mut extraout_dx: u16;
    let mut local_DX_47: u16;
    let mut extraout_dx_00: u16;
    let mut extraout_dx_01: u16;
    let mut local_DX_314: u16;
    let local_bx_6: *mut Struct725;
    let mut local_es_6: u16;
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

    local_es_6 = (param_1 >> 0x10);
    local_bx_6 = param_1;
    i_var1 = local_bx_6.field_0xc;
    if (i_var1 == 0x74) {
        i_var1 = local_bx_6.field_0x24;
        if (i_var1 == 1) {}
        // goto LAB_1020_e0ae;
        if (i_var1 != 6) {}
        // goto LAB_1020_e0b9;
        local_166 = 1;
    } else {
        if (i_var1 == 0x78) {
            i_var1 = local_bx_6.field_0x24;
            i_var4 = i_var1 + -1;
            if (i_var4 != 0) {
                if ((0 < i_var4) && (!SBORROW2(i_var4, 1))) {
                    if (i_var1 == 5 || i_var1 + -2 < 3) {
                        pass1_1020_e49a(local_bx_6, local_es_6);
                        local_DX_47 = extraout_dx_01;
                    } else {
                        if (i_var1 == 6) {
                            pass1_1020_e39c(param_1, 6);
                            pass1_1020_dca8(local_bx_6, local_es_6);
                            local_DX_47 = extraout_dx_00;
                        }
                    }
                }
                // goto LAB_1020_e0b9;
            }
            local_166 = 0x6a;
        } else {
            if (i_var1 == 0x79) {
                pass1_1020_e49a(local_bx_6, local_es_6);
                return;
            }
            // LAB_1020_e0ae:
            local_166 = 0x47;
        }
    }
    pass1_1020_e39c(param_1, local_166);
    local_DX_47 = extraout_dx;
    // LAB_1020_e0b9:
    u_var3 = pass1_1028_b58e(param_1);
    i_var1 = CONCAT31(extraout_var, u_var3);
    _local_6 = CONCAT31(extraout_var, u_var3) & 0xffff | local_DX_47 << 0x10;
    local_a = (i_var1 + 0x2e);
    u_var2 = (i_var1 + 0x30);
    if (local_bx_6.field_0xc != 0x79) {
        local_16c = u_var2;
        uStack363 = (u_var2 >> 8);
        pass1_1038_5804(
            CONCAT13(uStack363, CONCAT12(local_16c, local_a)),
            1,
            &dos_alloc_addr_1050_0002,
        );
    }
    if (local_bx_6.field_0x24 == 6) {
        local_16a = (local_a >> 0x10);
        pass1_1038_43cc(local_a, CONCAT22(1, local_16a), 2);
    }
    local_10 = (local_6 + 0xc);
    local_c = (local_6 + 0x10);
    local_2a = &local_10;
    if ((local_bx_6.field_0x24 == 6) && (local_c == 0)) {
        return;
    }
    temp_5ffa9150db = local_bx_6.field_0x28;
    local_14 = (temp_5ffa9150db + 0x20);
    puVar6 = &local_16;
    local_DX_314 = &local_18;
    pa_var5 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_14, (local_14 >> 0x10));
    pass1_1030_5b1c(
        CONCAT22(local_DX_314, pa_var5),
        CONCAT22(local_SS__1, &local_18),
        CONCAT22(local_SS__1, puVar6),
    );
    pass1_1028_c8ee(
        local_bx_6,
        local_es_6,
        local_bx_6.field_0x24,
        CONCAT22(local_SS__1, &local_10),
    );
    pass1_1008_3eb4(
        CONCAT22(local_SS__1, &local_10),
        CONCAT22(local_SS__1, &local_22),
        CONCAT22(local_SS__1, local_20),
        CONCAT22(local_SS__1, local_1e),
    );
    if (local_bx_6.field_0x24 == 1) {
        if (local_18 < local_22) {
            pass1_1030_5b3e(CONCAT22(local_DX_314, pa_var5), local_22, local_16);
            pass1_1030_5b1c(
                CONCAT22(local_DX_314, pa_var5),
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
        pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(local_SS__1, &local_158));
        local_158 = ctx.s_1_1050_389a;
        local_156 = &ctx.PTR_LOOP_1050_1008;
    }
    pass1_1028_ccd0(param_1, CONCAT22(local_SS__1, &local_10));
    return;
}

pub fn pass1_1020_e294(param_1: *mut Struct44) {
    let u_var1: u8;
    let extraout_var: u32;
    let mut local_eax_110: u32;
    let mut local_DX_44: u16;
    let mut local_DX_110: u16;
    let mut u_var2: u16;
    let mut local_es_6: u16;
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
    let mut u_stack12: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let mut temp_5fc08ad9c9: u32;
    let mut temp_5f52d5638b: u32;

    local_es_6 = (param_1 >> 0x10);
    u_var2 = param_1;
    if ((1 < (u_var2 + 0x24)) && ((u_var2 + 0x24) < 6)) {
        temp_5f52d5638b = (u_var2 + 0x28);
        local_6 = (temp_5f52d5638b + 0x20);
        u_var1 = pass1_1028_b58e(param_1);
        local_a = CONCAT31(extraout_var, u_var1);
        local_10 = (local_a + 0xc);
        u_stack12 = (local_a + 0x10);
        local_8 = local_DX_44;
        pass1_1028_c8ee(
            u_var2,
            local_es_6,
            (u_var2 + 0x24),
            CONCAT22(local_SS__1, &local_10),
        );
        local_eax_110._0_2_ = &local_10;
        local_DX_110 = local_DX_44;
        pass1_1028_c89c(
            param_1,
            CONCAT22(local_SS__1, local_eax_110),
            CONCAT22(local_SS__1, local_150),
        );
        local_14 = local_eax_110;
        local_15e._3_1_ = (local_14 >> 0x18);
        if ((local_15e._3_1_ == '\0') && (local_14 == 9)) {
            (u_var2 + 0x14) = 1;
        }
        local_18 = (local_a + 0x2e);
        local_1c = (local_18 + 4);
        pass1_1028_87f0(
            CONCAT22(local_SS__1, &local_140),
            0,
            (u_var2 + 0x14) + 1,
            0x79,
            &local_10,
            local_SS__1,
            local_1c,
            local_6,
        );
        pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(local_SS__1, &local_140));
    }
    (u_var2 + 0x26) = 1;
    return;
}

pub fn pass1_1020_e39c(param_1: u32, param_2: u16) {
    let mut u_var1: u32;
    let u_var2: u8;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let extraout_var: u32;
    let mut in_dx: i32;
    let mut unaff_ss: u16;
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

    u_var2 = pass1_1028_b58e(param_1);
    i_var3 = CONCAT31(extraout_var, u_var2);
    _local_6 = CONCAT31(extraout_var, u_var2) & 0xffff | in_dx << 0x10;
    local_c = (i_var3 + 0xc);
    local_8 = (i_var3 + 0x10);
    if (local_8 < 1) {
        u_var4 = 5;
    } else {
        u_var4 = 6;
    }
    (i_var3 + 0x14) = u_var4;
    u_var1 = (param_1 + 0x28);
    local_10 = (u_var1 + 0x20);
    local_14 = (i_var3 + 0x2e);
    local_18 = (local_14 + 4);
    pass1_1028_87f0(
        CONCAT22(unaff_ss, &local_13c),
        0,
        1,
        param_2,
        &local_c,
        unaff_ss,
        local_18,
        local_10,
    );
    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_13c));
    return;
}

pub fn pass1_1020_e44c(param_1: *mut Struct726) {
    let pi_var1: *mut i32;
    let local_bx_3: *mut Struct726;
    let local_es_3: *mut Struct726;

    local_es_3 = (param_1 >> 0x10);
    local_bx_3 = param_1;
    if (local_bx_3.field_0x12 == 2) {
        pi_var1 = &local_bx_3.field_0x14;
        unsafe { *pi_var1 = *pi_var1 + -1 };
        if ((local_bx_3.field_0x26 == 0) && (local_bx_3.field_0xc == 0x78)) {
            pass1_1020_e294(param_1);
        }
        if (local_bx_3.field_0x14 == 0) {
            pass1_1020_e08e(param_1);
            return;
        }
        if (local_bx_3.field_0x24 == 6) {
            local_bx_3.field_0xe = 0x49;
        }
    }
    return;
}

pub fn pass1_1020_e49a(param_1: *mut Struct44) {
    let mut i_var1: i32;
    let u_var2: u8;
    let extraout_AH: u8;
    let mut in_dx: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var2 = pass1_1028_b58e(param_1);
    i_var1 = (CONCAT11(extraout_AH, u_var2) + 0x14);
    local_a = 0;
    if (i_var1 == 6) {
        local_a = 9;
    } else {
        if (i_var1 == 7) {
            local_a = 6;
        } else {
            if (i_var1 == 8) {
                local_a = 7;
            } else {
                if (i_var1 == 9) {
                    local_a = 8;
                }
            }
        }
    }
    pass1_1020_e39c(param_1, local_a);
    return;
}

pub fn pass1_1020_e4fa(param_1: u32, param_2: u16) {
    let u_var1: u8;
    let extraout_AH: u8;
    let mut in_dx: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    match (param_2) {
        2 | 5 | 6 | 7 => local_4 = 4,
        3 | 8 => local_4 = 5,
        // default:
        _ => {
            u_var1 = pass1_1028_b58e(param_1);
            local_4 = (CONCAT11(extraout_AH, u_var1) + 0x14) + 2;
        }
    }
    return local_4;
}

pub fn pass1_1020_e558(param_1: *mut Struct44) {
    let u_var1: u8;
    let pu_var2: *mut u32;
    let paVar3: *mut Struct493;
    let extraout_var: u32;
    let mut in_dx: u16;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let mut unaff_ss: u16;
    let mut local_30: u32;
    let mut local_24: [u8; 12];
    let mut local_18: u32;
    let mut local_14: u32;
    let mut local_10: u32;
    let mut u_stack12: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var6 = (param_1 >> 0x10);
    u_var5 = param_1;
    if ((u_var5 + 0xc) == 0x79) {
        (u_var5 + 0x14) = (u_var5 + 0x24);
        (u_var5 + 0x24) = 0;
    }
    if ((u_var5 + 0x24) != 6) {
        u_var1 = pass1_1028_b58e(param_1);
        local_6 = CONCAT31(extraout_var, u_var1);
        local_a = (local_6 + 0x14);
        local_8 = local_a;
        local_4 = in_dx;
        pass1_1020_e4fa(param_1, local_a);
        local_10 = (local_6 + 0xc);
        u_stack12 = (local_6 + 0x10);
        local_18 = CONCAT22(local_18._2_2_, &local_10);
        pass1_1028_c8ee(
            u_var5,
            u_var6,
            (u_var5 + 0x24),
            CONCAT22(unaff_ss, &local_10),
        );
        pu_var2 = &local_10;
        pass1_1028_c89c(
            param_1,
            CONCAT22(unaff_ss, pu_var2),
            CONCAT22(unaff_ss, local_24),
        );
        unsafe { local_18 = *pu_var2 };
        u_var4 = (pu_var2 + 2);
        local_30._3_1_ = (local_18 >> 0x18);
        local_14._0_2_ = local_18;
        local_14 = local_18;
        if (local_30._3_1_ != '\0') {
            paVar3 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_14, u_var4);
            local_14._0_2_ = &paVar3.field_0x14;
        }
        local_8 = local_14;
        pass1_1020_e4fa(param_1, local_14);
        (u_var5 + 0x14) = local_a + local_14;
        return;
    }
    (u_var5 + 0x14) = 1;
    return;
}

pub fn pass1_1020_e652(param_1: u32, param_2: u16, param_3: u16, param_4: u32) -> i32 {
    let mut i_var1: i32;
    let mut u_var2: u16;
    let mut unaff_ss: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let mut uStack4: u16;

    local_8 = param_2;
    uStack4 = (param_2 + 4);
    u_var2 = (param_1 >> 0x10);
    pass1_1028_c8ee(
        param_1,
        u_var2,
        (param_1 + 0x24),
        CONCAT22(unaff_ss, &local_8),
    );
    i_var1 = pass1_1028_c7b6(param_1, &local_8, unaff_ss, param_4);
    if (i_var1 != 0) {
        i_var1 = 1;
    }
    return i_var1;
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
    let mut u_var1: u16;

    if (param_1 == 0xf) {
        u_var1 = 1;
    } else {
        u_var1 = 3;
    }
    return u_var1;
}

pub fn pass1_1020_c444(param_1: *mut *mut Struct706, param_2: u32, param_3: u32) {
    let local_bx_20: *mut Struct706;
    let mut u_var1: u16;

    pass1_1030_1cd8(param_1, param_2, param_3);
    u_var1 = (param_1 >> 0x10);
    local_bx_20 = param_1;
    local_bx_20.field_0x18 = 0;
    local_bx_20.field_0x1c = 0;
    unsafe { *param_1 = 0xc834 };
    local_bx_20.field_0x2 = 0x1020;
    return;
}

pub fn pass1_1020_c47a(param_1: *mut Struct44) {
    Struct44 * *local_es_4;

    local_es_4 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0xc834;
    (param_1 + 2) = 0x1020;
    error_check_1000_17ce((param_1 + 0x18));
    pass1_1030_1d28(param_1);
    return;
}

pub fn pass1_1020_c4a8(param_1: *mut Struct709, param_2: u32, param_3: u32, param_4: u16) {
    let mut u_var1: u32;
    let local_bx_4: *mut Struct709;
    let local_bx_39: *mut Struct710;
    let local_es_4: *mut Struct709;
    let mut u_var2: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_es_4 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    if (local_bx_4.field_0x1c != 0) {
        pass1_1020_c6a4((param_1 & 0xffff | ZEXT24(local_es_4) << 0x10));
    }
    u_var1 = local_bx_4.field_0x18;
    u_var2 = (u_var1 >> 0x10);
    local_bx_39 = (u_var1 + param_4 * 6);
    param_3 = (local_bx_39).field_0x0;
    param_2 = local_bx_39.field_0x4;
    return;
}

pub fn pass1_1020_c4f4(param_1: u32, param_2: u16, param_3: u16, param_2_00: u32) {
    let paVar1: *mut Struct493;
    let local_AX_43: *mut Struct712;
    let mut u_var2: i32;
    let mut u_var3: i32;
    let lVar4: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    lVar4 = pass1_1020_c6de(param_1, param_2_00);
    u_var2 = (lVar4 >> 0x10);
    u_var3 = u_var2 | lVar4;
    if (lVar4 != 0) {
        paVar1 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, param_2_00, (param_2_00 >> 0x10));
        local_AX_43._0_1_ = pass1_1030_6fa0(CONCAT22(u_var3, paVar1));
        local_AX_43 = CONCAT11(local_AX_43._1_1_, local_AX_43);
        (lVar4 + 4) = (local_AX_43 * 2 + 0x4ea4);
    }
    return;
}

pub fn pass1_1020_c538(param_1: *mut u8) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 0x12), (param_1 + 0x10));
}

pub fn pass1_1020_c54a(param_1: *mut Struct709) {
    let local_es_5: *mut Struct709;
    let mut local_6: u32;

    local_es_5 = (param_1 >> 0x10);
    if ((param_1 + 0x1c) != 0) {
        pass1_1020_c6a4((param_1 & 0xffff | ZEXT24(local_es_5) << 0x10));
    }
    return;
}

pub fn pass1_1020_c5b4(param_1: *mut u32, param_2: u32) {
    let plVar1: *mut long;
    let ppc_var2: fn();
    let u_var3: u8;
    let paVar4: *mut Struct493;
    let mut u_var5: i32;
    let mut u_var6: i32;
    let extraout_var: u32;
    let mut in_dx: u16;
    let mut extraout_dx: i32;
    let mut extraout_dx_00: i32;
    let mut u_var7: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    paVar4 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, param_2, (param_2 >> 0x10));
    u_var3 = pass1_1030_6fa0(CONCAT22(in_dx, paVar4));
    u_var5 = CONCAT31(extraout_var, u_var3);
    u_var6 = u_var5;
    pass1_1020_c6de(param_1, 0);
    _local_c = CONCAT22(extraout_dx, u_var6);
    u_var7 = (param_1 >> 0x10);
    if ((extraout_dx | u_var6) == 0) {
        unsafe {
            ppc_var2 = (*param_1 + 0x20);
            ppc_var2()
        };
        pass1_1020_c6de(param_1, 0);
        _local_c = CONCAT22(extraout_dx_00, u_var6);
        if ((extraout_dx_00 | u_var6) == 0) {
            return;
        }
    }
    (param_1 + 0x1c) = 1;
    plVar1 = (param_1 + 8);
    unsafe {
        *plVar1 = *plVar1 + 1;
        *_local_c = param_2;
        (_local_c + 4) = (u_var5 * 2 + 0x4ea4);
    }
    return;
}

pub fn pass1_1020_c644(in_struct_1: *mut Struct713, param_2: u16, param_3: u32) {
    let plVar1: *mut long;
    let mut u_var2: u16;
    let local_AX_39: *mut Struct714;
    let local_struct_1: *mut Struct713;
    let local_struct_1_hi: *mut Struct713;
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
    u_var2 = local_struct_1.field_0x1a;
    _local_6 = CONCAT22(u_var2, local_AX_39);
    plVar1 = &local_struct_1.field_0x8;
    unsafe {
        *plVar1 = *plVar1 + 1;
        *_local_6 = param_3
    };
    local_AX_39.field_0x4 = param_2;
    return;
}

pub fn pass1_1020_c694(param_1: *mut Struct709) {
    pass1_1020_c6a4(param_1);
    return;
}

pub fn pass1_1020_c6a4(param_1: *mut Struct709) {
    let local_struct_1: *mut Struct709;
    let local_es_3: *mut Struct709;

    local_es_3 = (param_1 >> 0x10);
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

pub fn pass1_1020_c6de(in_struct_1: *mut Struct715, param_2: u32) {
    let pu_var1: *mut u32;
    let mut u_var2: u32;
    let local_struct_1: *mut Struct715;
    let local_struct_1_hi: *mut Struct715;
    let mut local_6: u32;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    if (local_struct_1.field_0x1c != 0) {
        pass1_1020_c6a4((in_struct_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10));
    }
    local_6 = 0;
    while (true) {
        pu_var1 = &local_struct_1.field_0x10;
        let pu_var1_val = unsafe { *pu_var1 };
        if (pu_var1_val < local_6 || pu_var1_val == local_6) {
            return;
        }
        u_var2 = local_struct_1.field_0x18;
        if ((u_var2 + local_6 * 6) == param_2) {
            break;
        }
        local_6 = local_6 + 1;
    }
    return;
}

pub fn pass1_1020_c73a(in_struct_1: *mut Struct716) {
    let pu_var1: *mut u32;
    let paVar2: *mut Struct94;
    let mut u_var3: i32;
    let extraout_dx: *mut u16;
    let local_struct_1: *mut Struct716;
    let local_struct_1_hi: *mut Struct716;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_5ffbaa3a02: *mut Struct717;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    if (&local_struct_1.field_0x10 == 0) {
        paVar2 = local_struct_1.field_0xc;
        g_u16_ptr_1050_5f2e = local_struct_1.field_0xe;
    } else {
        temp_5ffbaa3a02 = local_struct_1.field_0x10;
        pu_var1 = &local_struct_1.field_0x14;
        let pu_var1_val = unsafe { *pu_var1 };
        paVar2 = (temp_5ffbaa3a02 + pu_var1_val);
        g_u16_ptr_1050_5f2e = (local_struct_1.field_0x12
            + local_struct_1.field_0x16
            + CARRY2(temp_5ffbaa3a02, pu_var1_val));
    }
    _local_6 = CONCAT22(g_u16_ptr_1050_5f2e, paVar2);
    if (&local_struct_1.field_0x18 == 0) {
        if (__g_Struct94_ptr_1 == 0) {
            _g_Struct94_ptr_1 = paVar2;
            struct_fn_1000_160a();
        } else {
        }
        u_var3 = paVar2 * 6;
        alloc_mem_1000_1708(u_var3, 0, 1, _g_Struct94_ptr_1, g_u16_ptr_1050_5f2e);
    } else {
        u_var3 = paVar2 * 6;
        alloc_mem_1000_0ed4(
            1,
            u_var3,
            (g_u16_ptr_1050_5f2e * 3 + CARRY2(paVar2, paVar2) + CARRY2(paVar2 * 2, paVar2)) * 2
                + CARRY2(paVar2 * 3, paVar2 * 3),
            &local_struct_1.field_0x18,
        );
        g_u16_ptr_1050_5f2e = extraout_dx;
    }
    _local_a = CONCAT22(g_u16_ptr_1050_5f2e, u_var3);
    if ((g_u16_ptr_1050_5f2e | u_var3) != 0) {
        &local_struct_1.field_0x10 = _local_6;
        &local_struct_1.field_0x18 = _local_a;
    }
    local_struct_1.field_0x1c = 1;
    return;
}

pub fn pass1_1020_c7fa(param_1: *mut u8, param_2: u32) -> i32 {
    return (param_1 + 4) - (param_2 + 4);
}

pub fn pass1_1020_c80e(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1020_c47a(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_c835() {
    let pbVar1: *mut u8;
    let pc_var2: *mut libc::c_char;
    let mut bVar3: u8;
    let mut cVar5: u8;
    let mut u_var6: u32;
    let mut in_CH: u8;
    let mut in_DL: u8;
    let mut in_DH: u8;
    let mut in_BX: i32;
    let pu_var7: *mut u16;
    let mut i_var8: i32;
    let unaff_BP: *mut u16;
    let mut unaff_si: i32;
    let mut unaff_ss: u16;
    let mut u_var9: u16;
    let mut in_CF: u8;
    let mut bVar10: bool;
    let mut in_stack_000020c3: u8;
    let mut bVar4: u8;

    pu_var7 = &stack0xfffe;
    cVar5 = '\t';
    while {
        unaff_BP = unaff_BP + -1;
        pu_var7 = pu_var7 + -1;
        unsafe { *pu_var7 = *unaff_BP };
        cVar5 = cVar5 + -1;
        '\0' < cVar5
    } {}
    u_var9 = (*(in_BX + unaff_si) >> 0x10);
    bVar10 = CARRY1(in_stack_000020c3, in_CH) || CARRY1(in_stack_000020c3 + in_CH, in_CF);
    pbVar1 = (s_514a_bmp_1050_20c5 + unaff_si);
    unsafe {
        bVar3 = *pbVar1;
        bVar4 = *pbVar1;
        *pbVar1 = bVar4 + in_DH + bVar10;
        pc_var2 = (in_BX + unaff_si);
        *pc_var2 =
            *pc_var2 + (in_BX >> 8) + (CARRY1(bVar3, in_DH) || CARRY1(bVar4 + in_DH, bVar10));
        u_var6 = (in_BX + unaff_si);
        u_var9 = (u_var6 >> 0x10);
        i_var8 = u_var6;
        (i_var8 + -2) = u_var9;
        pbVar1 = (in_BX + unaff_si);
        *pbVar1 = *pbVar1 ^ in_DL;
        (i_var8 + -4) = unaff_ss;
        pbVar1 = (in_BX + unaff_si);
        *pbVar1 = *pbVar1 ^ in_DL;
    }
    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
}

pub fn pass1_1020_c860(param_1: *mut u8) {
    let mut local_es_3: u16;

    local_es_3 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 6), (param_1 + 4));
}

pub fn pass1_1020_c872(param_1: u32, param_2: u32, param_3: u32) {
    let pu_var1: *mut u32;
    let pu_var2: *mut u32;
    let pi_var3: *mut i32;
    let local_struct_1: *mut Struct501;
    let mut u_var4: i32;
    let mut extraout_dx: i32;
    let local_bx_126: *mut Struct502;
    let mut i_var6: i32;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut bVar10: bool;
    let mut local_2a: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u32;
    let pu_var5: *mut u8;

    pu_var5 = _PTR_LOOP_1050_4fb8;
    alloc_mem_1000_07fc(_PTR_LOOP_1050_4fb8);
    u_var4 = pu_var5;
    local_6 = pu_var5 & 0xffff | extraout_dx << 0x10;
    if ((extraout_dx | u_var4) == 0) {
        local_6 = 0;
    } else {
        local_6 = ctx.s_1_1050_389a;
        (u_var4 + 2) = &ctx.PTR_LOOP_1050_1008;
        (u_var4 + 4) = 0;
        (u_var4 + 8) = 0;
        local_6 = s__s__s__1050_5bc0;
        (u_var4 + 2) = &ctx.PTR_LOOP_1050_1008;
        (u_var4 + 0xe) = 0;
        (u_var4 + 0xc) = 0;
        local_6 = 0xc9e6;
        (u_var4 + 2) = 0x1020;
    }
    if (local_6 == 0) {
        return;
    }
    u_var7 = (local_6 >> 0x10);
    local_bx_126 = local_6;
    local_bx_126.field_0x8 = param_3;
    local_bx_126.field_0xc = param_2;
    u_var8 = (param_1 >> 0x10);
    i_var6 = param_1;
    local_e = (i_var6 + 4);
    u_var9 = (i_var6 + 6);
    if ((i_var6 + 8) == 0) {
        // LAB_1020_c92d:
        local_bx_126.field_0x4 = (i_var6 + 4);
    } else {
        pu_var1 = (local_e + 0xe);
        unsafe {
            bVar10 = *pu_var1 < param_2._2_2_;
            if ((bVar10 || *pu_var1 == param_2._2_2_)
                && (bVar10
                    || (
                        pu_var1 = (local_e + 0xc),
                        *pu_var1 < param_2 || *pu_var1 == param_2,
                    )))
            {}
        }
        // goto LAB_1020_c92d;
        bVar10 = false;
        while (true) {
            if (local_e == 0) {
                break;
            }
            u_var9 = (local_e >> 0x10);
            pu_var2 = (local_e + 0xc);
            let pu_var2_val = unsafe { *pu_var2 };
            if (pu_var2_val < param_2 || pu_var2_val == param_2) {
                bVar10 = true;
                local_bx_126.field_0x4 = local_e;
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
    u_var9 = (param_1 >> 0x10);
    (param_1 + 4) = local_bx_126;
    (param_1 + 6) = u_var7;
    // LAB_1020_c9ab:
    unsafe {
        pi_var3 = (i_var6 + 8);
        *pi_var3 = *pi_var3 + 1;
    }
    return;
}

pub fn call_free_mem_1020_c9ba(param_1: *mut u16, param_2: u8) {
    unsafe {
        *param_1 = ctx.s_1_1050_389a;
    }
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
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

pub fn pass1_1020_ca0c(param_1: *mut Struct743, param_2: u16, param_3: u16, param_3_00: *mut u8) {
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
    u_var1 = pass1_1028_b4f2(CONCAT22(param_2, param_1));
    if ((u_var1 + 0x200) != 0x8000002) {
        local_struct_2 =
            process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000ffee, 8));
        pass1_1010_988c(local_struct_2, (param_1 + 0xc));
    }
    return;
}

pub fn pass1_1020_ca82(param_1: *mut u8) {
    let mut u_var1: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1028_be9e(param_1);
    u_var1 = pass1_1028_b4f2(param_1);
    if ((u_var1 + 0x200) != 0x8000002) {
        if ((param_1 + 0x12) == 5) {
            pass1_1020_cac2(param_1);
        }
    }
    return;
}

pub fn pass1_1020_cac2(param_1: *mut Struct44) {
    let pu_var1: *mut u16;
    let mut i_var2: i32;
    let ppc_var3: fn();
    let u_var4: u8;
    let local_AX_96: *mut Struct718;
    let mut u_var5: u16;
    let extraout_var: u32;
    let mut extraout_dx: i32;
    let mut u_var6: i32;
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let pp_var8: *mut pass1_struct_1;
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

    pp_var8 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000ffca, 2));
    local_4 = (pp_var8 >> 0x10);
    local_6 = pp_var8;
    local_8 = u16_1050_13ae;
    if (u16_1050_13ae == 1) {
        local_8 = 2;
    }
    _local_c = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000ffca, 8));
    u_var7 = (_local_c >> 0x10);
    local_10 = (_local_c + 10);
    local_14 = (_local_c + 0xe);
    pass1_1008_5784(CONCAT22(unaff_ss, &stack0xffe4), local_10);
    loop {
        while {
            loop {
                while {
                    local_AX_96 = &stack0xffe4;
                    pass1_1008_5b12(CONCAT22(unaff_ss, local_AX_96));
                    u_var6 = extraout_dx | local_AX_96;
                    if (u_var6 == 0) {
                        return;
                    }
                    i_var2 = local_AX_96.field_0x4;
                    (i_var2 < 0x12) || (SBORROW2(i_var2, 0x12))
                } {}
                if (i_var2 != 0x13 && 0 < i_var2 + -0x12) {
                    break;
                }
                local_34 = 0;
                if (local_8 == 3) {
                    local_34 = local_AX_96.field_0x6 / 2;
                } else {
                    if (local_8 == 4) {
                        i_var2 = local_AX_96.field_0x6 * 3;
                        local_34 = (i_var2 + (i_var2 >> 0xf & 3)) >> 2;
                    }
                }
                pu_var1 = &local_AX_96.field_0x6;
                unsafe { *pu_var1 = *pu_var1 - local_34 };
                u_var7 = (local_10 >> 0x10);
                (local_10 + 10) = 0;
                ppc_var3 = (local_10 + 0xc);
                (**ppc_var3)(
                    &ctx.PTR_LOOP_1050_1008,
                    local_10,
                    u_var7,
                    local_AX_96,
                    extraout_dx,
                );
                (local_10 + 10) = 1;
                local_18 = 0;
                ppc_var3 = (local_14 + 4);
                (**ppc_var3)(
                    &ctx.PTR_LOOP_1050_1008,
                    local_14,
                    (local_14 >> 0x10),
                    local_AX_96,
                    extraout_dx,
                );
            }
            i_var2 != 0x81
        } {}
        local_24 = 0;
        if (local_8 == 2) {
            u_var5 = local_AX_96.field_0x6;
            // LAB_1020_cba7:
            u_var6 = u_var5 >> 0xf & 3;
            local_24 = (u_var5 + u_var6) >> 2;
        } else {
            if (local_8 == 3) {
                u_var5 = local_AX_96.field_0x6;
                u_var6 = u_var5 >> 0xf;
                local_24 = u_var5 / 2;
            } else {
                if (local_8 == 4) {
                    u_var5 = local_AX_96.field_0x6 * 3;
                    // goto LAB_1020_cba7;
                }
            }
        }
        u_var4 = pass1_1028_b58e(param_1);
        pass1_1030_7ddc(
            CONCAT31(extraout_var, u_var4) & 0xffff | u_var6 << 0x10,
            (local_AX_96.field_0x6 - local_24),
            0x1e,
        );
        ppc_var3 = (local_10 + 0xc);
        (**ppc_var3)(
            0x1030,
            local_10,
            (local_10 >> 0x10),
            local_AX_96,
            extraout_dx,
        );
        local_18 = 0;
    }
}

pub fn pass1_1020_cc56(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
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

pub fn pass1_1020_cd30(param_1: *mut Struct719) {
    let local_bx_3: *mut Struct719;
    let local_es_3: *mut Struct719;

    local_es_3 = (param_1 >> 0x10);
    local_bx_3 = param_1;
    if (((local_bx_3.field_0x12 == 6) || (local_bx_3.field_0x12 == 5))
        && ((local_bx_3.field_0x1a & 2) != 0))
    {
        return 1;
    }
    return 0;
}

pub fn pass1_1020_cd58(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
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

pub fn pass1_1020_ce08(param_1: *mut Struct743, param_2: u16, param_3: u16, param_3_00: *mut u8) {
    pass1_1028_0982(param_1, CONCAT22(param_3, param_2), param_3_00);
    CONCAT22(param_2, param_1) = 0xd004;
    param_1.field_0x2 = 0x1020;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1020_ce32(param_1: i32, param_2: u16, param_3: u16) {
    let mut u_var1: u32;
    let ppVar2: *mut pass1_struct_1;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let u_var5: u8;
    let u_var6: u8;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut in_stack_0000ffea: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1028_09b8(param_1, param_2, param_3);
    u_var1 = pass1_1028_b4f2(CONCAT22(param_2, param_1));
    if ((u_var1 + 0x200) != 0x8000002) {
        ppVar2 =
            process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000ffea, 8));
        pass1_1010_988c(ppVar2, (param_1 + 0xc));
        u_var9 = 0;
        u_var10 = 9;
        u_var7 = 1;
        u_var8 = 0;
        u_var4 = 0;
        u_var5 = 0;
        u_var6 = 0;
        u_var3 = 0;
        ppVar2 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, 0x37);
        post_win_msg_1008_a0e4(
            ppVar2,
            CONCAT22(u_var4, u_var3),
            CONCAT11(u_var6, u_var5),
            u_var7,
            CONCAT22(u_var9, u_var8),
            u_var10,
        );
    }
    return;
}

pub fn pass1_1020_ce9e(param_1: *mut u8) {
    let mut u_var1: u16;
    let mut u_var2: u32;
    let ppVar3: *mut pass1_struct_1;
    let mut in_stack_0000ffea: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1028_be9e(param_1);
    if ((param_1 + 0x12) == 5) {
        pass1_1020_cefc(param_1);
        u_var2 = pass1_1028_b4f2(param_1);
        u_var1 = (u_var2 >> 0x10);
        if ((u_var2 + 0x200) != 0x8000002) {
            ppVar3 = process_struct_1010_20ba(
                ctx._g_Struct372_1050_0ed0,
                CONCAT22(in_stack_0000ffea, 0x2b),
            );
            pass1_1010_043a(ppVar3, (u_var2 + 4), 5);
        }
    }
    return;
}

pub fn pass1_1020_cefc(param_1: *mut u8) {
    let pu_var1: *mut u8;
    let ppVar2: *mut pass1_struct_1;
    let mut in_stack_0000ffec: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pu_var1 = pass1_1028_b4f2(param_1);
    if ((pu_var1 + 0x200) == 0x8000002) {
        local_c = 0x32;
    } else {
        ppVar2 =
            process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000ffec, 8));
        local_c = pass1_1010_96c2(ppVar2);
        if (0x32 < local_c) {
            local_c = 0x32;
        }
        pass1_1010_96a8(ppVar2, local_c);
    }
    pass1_1020_cf6c(param_1, (param_1 >> 0x10), local_c, pu_var1);
    return;
}

pub fn pass1_1020_cf6c(param_1: u16, param_2: u16, param_1_00: i32, param_2_00: *mut u8) {
    Struct721 * *ppaVar1;
    let pu_var2: *mut u32;
    Struct722 * *ppaVar3;
    let paVar4: *mut Struct721;
    let mut u_var5: i32;
    let mut u_var6: u32;
    let mut u_var7: u16;
    let local_AX_14: *mut Struct720;
    let mut u_var8: i32;
    let local_AX_42: *mut Struct721;
    let local_DX_60: *mut Struct722;
    let mut u_var9: u16;
    let mut local_a: u32;

    u_var9 = (param_2_00 >> 0x10);
    u_var6 = (param_2_00 + 0x1f6);
    local_AX_14 = u_var6;
    u_var7 = (u_var6 >> 0x10);
    u_var8 = param_1_00 / 5;
    local_AX_42 = (u_var8 * -4 + param_1_00);
    ppaVar1 = &local_AX_14.field_0x50;
    paVar4 = *ppaVar1;
    *ppaVar1 = local_AX_42 + *ppaVar1;
    pu_var2 = &local_AX_14.field_0x52;
    unsafe {
        *pu_var2 = *pu_var2 + (local_AX_42 >> 0xf) + CARRY2(paVar4, local_AX_42);
        local_DX_60 = (u_var8 >> 0xf);
        pu_var2 = &local_AX_14.field_0x78;
        u_var5 = *pu_var2;
        *pu_var2 = *pu_var2 + u_var8;
        ppaVar3 = &local_AX_14.field_0x7a;
        *ppaVar3 = local_DX_60 + (*ppaVar3 + CARRY2(u_var5, u_var8));
        pu_var2 = &local_AX_14.field_0xa0;
        u_var5 = *pu_var2;
        *pu_var2 = *pu_var2 + u_var8;
        ppaVar3 = &local_AX_14.field_0xa2;
        *ppaVar3 = local_DX_60 + (*ppaVar3 + CARRY2(u_var5, u_var8));
        pu_var2 = &local_AX_14.field_0xc8;
        u_var5 = *pu_var2;
        *pu_var2 = *pu_var2 + u_var8;
        ppaVar3 = &local_AX_14.field_0xca;
        *ppaVar3 = local_DX_60 + (*ppaVar3 + CARRY2(u_var5, u_var8));
        pu_var2 = &local_AX_14.field_0xf0;
        u_var5 = *pu_var2;
        *pu_var2 = *pu_var2 + u_var8;
        ppaVar3 = &local_AX_14.field_0xf2;
        *ppaVar3 = local_DX_60 + (*ppaVar3 + CARRY2(u_var5, u_var8));
        (param_2_00 + 0x1fe) = 1;
    }
    return;
}

pub fn pass1_1020_cfde(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
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
    let mut i_var1: i32;
    let mut u_var2: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    if ((param_1 + 0x12) != 6) {
        return;
    }
    u_var2 = pass1_1028_b4f2(param_1);
    if ((u_var2 + 0x200) != 0x8000002) {
        i_var1 = pass1_1028_cb04(param_1);
        if ((i_var1 == 0) || (i_var1 = pass1_1020_d194(param_1), i_var1 == 0)) {
            i_var1 = 6;
            // goto LAB_1020_d10b;
        }
        pass1_1028_c952(param_1);
    }
    i_var1 = 5;
    // LAB_1020_d10b:
    pass1_1028_bdac(param_1, i_var1);
    return;
}

pub fn pass1_1020_d118(param_1: *mut u8, param_2: u32, param_3: u32, param_4: u32) {
    let mut i_var1: i32;
    let mut u_var2: u16;
    let mut u_var3: u16;

    i_var1 = pass1_1028_c7b6(param_1, param_2, param_4);
    if (i_var1 == 5) {
        u_var2 = param_1;
        u_var3 = (param_1 >> 0x10);
        pass1_1028_c23e(u_var2, u_var3, param_2, param_3, param_4);
        if (i_var1 != 0) {
            pass1_1028_c3aa(u_var2, u_var3, param_2, param_3, param_4);
            if (i_var1 != 0) {
                u_var2 =
                    pass1_1028_c314(u_var2, u_var3, param_2, param_3, (param_3 >> 0x10), param_4);
                if (u_var2 != 0) {
                    return 1;
                }
            }
        }
    } else {
        PTR_LOOP_1050_50ca = 0x6a8;
    }
    return 0;
}

pub fn pass1_1020_d194(param_1: *mut Struct44) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let u_var4: u8;
    let pa_var5: *mut Struct493;
    let puVar6: *mut u8;
    let mut i_var7: i32;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let extraout_var: u32;
    let pu_var10: *mut u8;
    let mut u_var11: u32;
    let mut u_var12: i32;
    let mut u_var13: i32;
    let mut extraout_dx: i32;
    let mut extraout_dx_00: u16;
    let mut extraout_dx_01: u16;
    let mut extraout_dx_02: i32;
    let mut u_var14: u16;
    let mut unaff_ss: u16;
    let mut uVar15: u32;
    let ppVar16: *mut pass1_struct_1;
    let uVar17: u8;
    let uVar18: u8;
    let mut uVar19: u16;
    let mut u_var20: u16;
    let mut u_var21: u16;
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

    pass1_1030_bcae(local_4, unaff_ss);
    uVar15 = pass1_1028_b4f2(param_1);
    u_var12 = (uVar15 >> 0x10);
    u_var1 = (uVar15 + 0x10);
    pa_var5 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
    u_var13 = u_var12;
    u_var4 = pass1_1028_b58e(param_1);
    u_var3 = CONCAT31(extraout_var, u_var4) & 0xffff;
    puVar6 = local_4;
    pass1_1030_bd74(
        puVar6,
        unaff_ss,
        CONCAT22(u_var12, pa_var5),
        (CONCAT31(extraout_var, u_var4) & 0xffff | u_var13 << 0x10),
    );
    if (puVar6 < 0) {
        return;
    }
    if (0x1e < puVar6) {
        u_var20 = 0x87;
        ppVar16 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, 0x870009);
        i_var7 = ppVar16;
        pass1_1010_65d0(ppVar16, u_var20);
        if (i_var7 == 0) {
            u_var12 = extraout_dx;
            pu_var10 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 0x15);
            u_var8 = SUB42(pu_var10, 0);
            u_var14 = SUB42(&PTR_LOOP_1050_1038, 0);
            pass1_1038_4e78(uVar15, pu_var10 & 0xffff | u_var12 << 0x10);
            _local_22 = CONCAT22(extraout_dx_00, u_var8);
            ppc_var2 = (*_local_22 + 0x10);
            u_var9 = u_var8;
            uVar19 = u_var8;
            u_var21 = extraout_dx_00;
            ppc_var2(&PTR_LOOP_1050_1038, u_var8, extraout_dx_00);
            _local_26 = CONCAT22(extraout_dx_01, u_var9);
            local_2a = 0;
            while (true) {
                if (_local_26 <= local_2a) {
                    if (_local_22 == 0x0) {
                        return;
                    }
                    ppc_var2 = *_local_22;
                    ppc_var2(
                        u_var14,
                        u_var8,
                        extraout_dx_00,
                        1,
                        uVar19,
                        u_var21,
                        _local_22,
                        _local_22,
                    );
                    return;
                }
                uVar17 = u_var3;
                uVar18 = (u_var3 >> 8);
                u_var11 = _local_26;
                u_var12 = u_var13;
                pass1_1030_1d58(_local_22);
                puVar6 = local_4;
                u_var14 = 0x1030;
                pass1_1030_bd74(
                    puVar6,
                    unaff_ss,
                    (u_var11 & 0xffff | extraout_dx_02 << 0x10),
                    CONCAT22(u_var12, CONCAT11(uVar18, uVar17)),
                );
                if ((0 < puVar6) && (puVar6 < 0x1f)) {
                    break;
                }
                local_2a = local_2a + 1;
            }
            if (_local_22 == 0x0) {
                return;
            }
            ppc_var2 = *_local_22;
            ppc_var2(
                0x1030,
                u_var8,
                extraout_dx_00,
                1,
                uVar19,
                u_var21,
                _local_22,
                _local_22,
                u_var11,
                extraout_dx_02,
            );
            return;
        }
    }
    return;
}

pub fn pass1_1020_d2ee(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_d37c(param_1: *mut u16) {
    let mut u_var1: u16;

    pass1_1028_b354(param_1);
    u_var1 = (param_1 >> 0x10);
    unsafe {
        (param_1 + 0x20) = 0;
        *param_1 = 0xd53e;
        (param_1 + 2) = 0x1020;
    }
    return param_1;
}

pub fn pass1_1020_d3a4(param_1: *mut u16, param_2: u16, param_3: u16, param_4: u32) {
    let mut u_var1: u16;

    pass1_1028_b39e(param_1, param_3, param_4);
    u_var1 = (param_1 >> 0x10);
    unsafe {
        (param_1 + 0x20) = param_2;
        *param_1 = 0xd53e;
        (param_1 + 2) = 0x1020;
    }
    return param_1;
}

pub fn pass1_1020_a6ee(param_1: *mut u8, param_2: u16) {
    let local_AX__1: *mut Struct698;
    let paVar1: *mut Struct493;
    let local_AX_84: *mut Struct699;
    let mut in_dx: i32;
    let mut unaff_ss: u16;
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

    paVar1 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, 2, 0x400);
    _local_6 = CONCAT22(in_dx, paVar1);
    if (((in_dx | paVar1) == 0) || (&paVar1[0x11].field_0x2 == 0x8000002)) {
        _local_a = process_struct_1010_20ba(
            ctx._g_Struct372_1050_0ed0,
            CONCAT22(in_stack_0000fea0, 0x2f),
        );
        local_10 = (_local_a >> 0x10);
        local_e = (_local_a + 0x20);
        local_AX_84 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_e, (local_e >> 0x10));
        local_12 = local_AX_84;
        zero_list_1008_3e38(CONCAT22(unaff_ss, &local_18));
        local_1a = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, param_2, 0x28);
        if (local_1a != 0) {
            local_14 = 1;
        }
        pass1_1020_b2da(
            param_1,
            (local_1a != 0),
            CONCAT22(unaff_ss, &local_18),
            CONCAT22(local_10, local_12),
        );
        pass1_1028_87f0(
            CONCAT22(unaff_ss, &local_13e),
            0,
            0,
            param_2,
            &local_18,
            unaff_ss,
            (_PTR_LOOP_1050_4e70 + 4),
            (local_12 + 4),
        );
        pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_13e));
        if (local_1a != 0) {
            pass1_1020_ad90(CONCAT22(unaff_ss, &local_18), (local_12 + 4));
        }
        (local_1e + 0x1c) = 0x8000001;
    }
    return;
}

pub fn pass1_1020_a80e(param_1: u16, param_2: u16, param_2_00: i32) {
    let mut u_var1: u32;
    let local_AX__1: *mut Struct700;
    let paVar2: *mut Struct493;
    let paVar3: *mut Struct493;
    let mut in_dx: i32;
    let mut u_var4: u16;
    let ppVar5: *mut pass1_struct_1;
    let mut in_stack_0000ffe4: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, 2, 0x400);
    if (((in_dx | paVar2) == 0) || (&paVar2[0x11].field_0x2 == 0x8000002)) {
        ppVar5 = process_struct_1010_20ba(
            ctx._g_Struct372_1050_0ed0,
            CONCAT22(in_stack_0000ffe4, 0x2f),
        );
        u_var4 = (ppVar5 >> 0x10);
        u_var1 = (ppVar5 + 0x20);
        paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
        if (param_2_00 == 10) {
            call_infinite_loop_1020_b872(param_1, param_2, paVar2, u_var4);
            return;
        }
        paVar3 = paVar2;
        pass1_1020_b0aa(param_1, param_2, param_2_00);
        if (paVar3 != 0x0) {
            pass1_1020_abc0(param_1, param_2, paVar3, paVar2, u_var4);
        }
    }
    return;
}

pub fn pass1_1020_a89e(param_1: *mut u8, param_2: *mut u32, param_3: u16) {
    let pi_var1: *mut i32;
    let local_AX__1: *mut Struct701;
    let paVar2: *mut Struct493;
    let pu_var3: *mut u8;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let mut extraout_dx: i32;
    let mut unaff_si: u16;
    let mut unaff_ss: u16;
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

    _local_6 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x2f));
    u_var6 = (_local_6 >> 0x10);
    local_a = (_local_6 + 0x20);
    paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_a, (local_a >> 0x10));
    _local_e = CONCAT22(u_var6, paVar2);
    unsafe {
        local_14._0_4_ = *param_2;
        local_14._4_2_ = (param_2 + 1);
    }
    local_4c2 = local_14;
    pass1_1008_3e94(
        local_14,
        CONCAT22(unaff_ss, &local_18),
        CONCAT22(unaff_ss, &local_16),
    );
    pass1_1008_3e76(CONCAT22(unaff_ss, local_14), 0, local_18, local_16 + 2);
    pass1_1028_8888(
        CONCAT22(unaff_ss, &local_13c),
        0,
        0x7a,
        local_14,
        unaff_ss,
        0x8000002,
        0x4000002,
        local_a,
    );
    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_13c));
    pass1_1008_3e76(CONCAT22(unaff_ss, local_14), 0, local_18 - 2, local_16);
    pass1_1028_8888(
        CONCAT22(unaff_ss, &local_260),
        0,
        0x47,
        local_14,
        unaff_ss,
        0x8000002,
        0x4000002,
        local_a,
    );
    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_260));
    pass1_1008_3e76(CONCAT22(unaff_ss, local_14), 1, local_18 - 2, local_16);
    pass1_1028_8888(
        CONCAT22(unaff_ss, &local_384),
        0,
        0x6a,
        local_14,
        unaff_ss,
        0x8000002,
        0x4000002,
        local_a,
    );
    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_384));
    pass1_1020_ad90(CONCAT22(unaff_ss, local_14), local_a);
    pass1_1008_3e76(CONCAT22(unaff_ss, local_14), 1, local_18 - 2, local_16 + 1);
    pass1_1028_8888(
        CONCAT22(unaff_ss, &local_4a8),
        0,
        0x7f,
        local_14,
        unaff_ss,
        0x8000002,
        0x4000002,
        local_a,
    );
    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_4a8));
    pass1_1020_ad90(CONCAT22(unaff_ss, local_14), local_a);
    _local_4ac = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 8));
    local_4b0 = (_local_4ac + 0x12);
    pass1_1008_5784(CONCAT22(unaff_ss, local_4b8), local_4b0);
    local_4be = 0;
    loop {
        while {
            pu_var3 = local_4b8;
            pass1_1008_5b12(CONCAT22(unaff_ss, pu_var3));
            _local_4bc = CONCAT22(extraout_dx, pu_var3);
            if ((extraout_dx | pu_var3) == 0) {
                pass1_1010_9674(_local_4ac);
                return;
            }
            ((pu_var3 + 4) != 0x3e) && ((pu_var3 + 4) != 0x41)
        } {}
        while (0 < (_local_4bc + 6)) {
            if (local_4be == 0) {
                u_var5 = local_16 - 2;
                // LAB_1020_ab4a:
                u_var4 = local_18 - 2;
                // LAB_1020_ab51:
                local_4be = local_4be + 1;
                pass1_1008_3e76(CONCAT22(unaff_ss, local_14), 0, u_var4, u_var5);
            } else {
                if (local_4be == 1) {
                    u_var5 = local_16 + 2;
                    // goto LAB_1020_ab4a;
                }
                if (local_4be == 2) {
                    u_var5 = local_16 + 2;
                    // LAB_1020_ab6e:
                    u_var4 = local_18 + 2;
                    // goto LAB_1020_ab51;
                }
                if (local_4be == 3) {
                    u_var5 = local_16 - 2;
                    // goto LAB_1020_ab6e;
                }
                local_4be = local_4be + 1;
                pass1_1020_b2da(param_1, 0, CONCAT22(unaff_ss, local_14), _local_e);
            }
            pass1_1028_8888(
                CONCAT22(unaff_ss, &local_5ee),
                0,
                (_local_4bc + 4),
                local_14,
                unaff_ss,
                0x8000002,
                0x4000002,
                local_a,
            );
            pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_5ee));
            pi_var1 = (_local_4bc + 6);
            unsafe {
                *pi_var1 = *pi_var1 + -1;
            }
            local_5ee = ctx.s_1_1050_389a;
            local_5ec = &ctx.PTR_LOOP_1050_1008;
        }
    }
}

pub fn pass1_1020_abc0(param_1: *mut u8, param_2: u16, param_3: u32) {
    let mut u_var1: u16;
    let mut unaff_ss: u16;
    let mut local_12e: u16;
    let mut local_12c: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let mut local_4: u16;

    zero_list_1008_3e38(CONCAT22(unaff_ss, &local_8));
    local_a = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, param_2, 0x28);
    if (local_a != 0) {
        local_4 = 1;
    }
    pass1_1020_b2da(
        param_1,
        (local_a != 0),
        CONCAT22(unaff_ss, &local_8),
        param_3,
    );
    u_var1 = (param_3 >> 0x10);
    pass1_1028_87f0(
        CONCAT22(unaff_ss, &local_12e),
        0,
        0,
        param_2,
        &local_8,
        unaff_ss,
        (_PTR_LOOP_1050_4e70 + 4),
        (param_3 + 4),
    );
    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_12e));
    if (local_a != 0) {
        pass1_1020_ad90(CONCAT22(unaff_ss, &local_8), (param_3 + 4));
    }
    return;
}

pub fn pass1_1020_ac6e(param_1: *mut u8, param_2: u16, param_3: u16, param_4: u16) {
    let b_var1: bool;
    let pu_var2: *mut u8;
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
    _local_12 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_152, 0x2f));
    local_DX_124 = (_local_12 >> 0x10);
    local_16 = (_local_12 + 0x20);
    puStack342 = local_16;
    local_DX_154 = local_DX_124;
    local_1a = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, puStack342, (local_16 >> 0x10));
    local_18 = local_DX_154;
    b_var1 = pass1_1020_b1ae(param_1, CONCAT22(local_SS__1, local_e), *(local_1a + 4));
    if (b_var1 != 0) {
        pu_var2 = local_e;
        pass1_1020_b240(
            param_1,
            CONCAT22(local_SS__1, pu_var2),
            CONCAT22(local_18, local_1a),
        );
        if (pu_var2 != 0x0) {
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
            pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(local_SS__1, &local_146));
        }
    }
    return;
}

pub fn pass1_1020_ad90(param_1: *mut u8, param_2: u32) {
    let pp_var1: fn();
    let paVar2: *mut Struct493;
    let pu_var3: *mut u8;
    let mut i_var4: i32;
    let mut i_var5: i32;
    let mut extraout_dx: u16;
    let mut extraout_dx_00: i32;
    let mut u_var6: i32;
    let mut extraout_dx_01: u16;
    let mut unaff_ss: u16;
    let pu_var7: *mut u32;
    let mut u_var8: u16;
    let u_var9: u8;
    let u_var10: u8;
    let mut u_var11: u16;
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
        CONCAT22(unaff_ss, local_14),
        CONCAT22(unaff_ss, &local_6),
        CONCAT22(unaff_ss, &local_4),
    );
    pass1_1030_627e(_PTR_LOOP_1050_5740, param_1, param_2);
    local_26 = extraout_dx;
    local_12 = extraout_dx;
    paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_14, extraout_dx);
    _local_18 = CONCAT22(local_26, paVar2);
    local_1c = &paVar2[1].field_0x10;
    local_20 = (local_1c + 4);
    local_28 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, param_2, (param_2 >> 0x10));
    pu_var7 = pass1_1030_5b5c(local_28, local_26);
    unsafe { local_2e._0_4_ = *pu_var7 };
    local_2e._4_2_ = (pu_var7 + 4);
    local_4e = local_2e;
    pass1_1008_3e94(
        local_2e,
        CONCAT22(unaff_ss, &local_24),
        CONCAT22(unaff_ss, &local_22),
    );
    i_var4 = local_4 + 1;
    _local_c = CONCAT22(local_4 - 1, local_6 - 1);
    i_var5 = local_6 + 1;
    if ((local_4 - 1) < 0) {
        _local_c = (local_6 - 1);
    }
    if (local_22 <= i_var4) {
        i_var4 = local_22 - 1;
    }
    if (local_c < 0) {
        _local_c = _local_c & 0xffff0000;
    }
    if (local_24 <= i_var5) {
        i_var5 = local_24 - 1;
    }
    _local_10 = CONCAT22(i_var4, i_var5);
    zero_list_1008_6c90(local_3a, unaff_ss);
    pass1_1008_6cec(
        CONCAT22(unaff_ss, local_3a),
        local_8,
        _local_10,
        local_8,
        _local_c,
    );
    pu_var3 = local_3a;
    u_var6 = extraout_dx_00;
    pass1_1030_6522(_PTR_LOOP_1050_5740, CONCAT22(unaff_ss, pu_var3), param_2);
    _local_3e = CONCAT22(u_var6, pu_var3);
    if ((u_var6 | pu_var3) != 0) {
        local_42 = 0;
        local_44 = 0;
        local_46 = local_c;
        while (local_46 <= local_10) {
            local_4e = local_a;
            while (local_4e <= local_e) {
                pp_var1 = (*_local_3e + 4);
                u_var8 = local_44;
                local_44 = local_44 + 1;
                (**pp_var1)(0x1030, _local_3e, (_local_3e >> 0x10));
                local_42 = CONCAT22(extraout_dx_01, u_var8);
                local_42._3_1_ = (extraout_dx_01 >> 8);
                if (local_42._3_1_ == '\0') {
                    local_5a = u_var8;
                    if (u_var8 == 7) {
                        pass1_1008_3e76(param_1, local_8, local_46, local_4e);
                        u_var9 = local_20;
                        u_var10 = (local_20 >> 8);
                        u_var11 = (local_20 >> 0x10);
                        u_var8 = 6;
                    } else {
                        if (u_var8 == 8) {
                            pass1_1008_3e76(param_1, local_8, local_46, local_4e);
                            u_var9 = local_20;
                            u_var10 = (local_20 >> 8);
                            u_var11 = (local_20 >> 0x10);
                            u_var8 = 7;
                        } else {
                            if (u_var8 != 9) {}
                            // goto LAB_1020_af1c;
                            pass1_1008_3e76(param_1, local_8, local_46, local_4e);
                            u_var9 = local_20;
                            u_var10 = (local_20 >> 8);
                            u_var11 = (local_20 >> 0x10);
                            u_var8 = 8;
                        }
                    }
                    pass1_1028_87f0(
                        CONCAT22(unaff_ss, &local_17e),
                        0,
                        0,
                        u_var8,
                        param_1,
                        (param_1 >> 0x10),
                        CONCAT22(u_var11, CONCAT11(u_var10, u_var9)),
                        param_2,
                    );
                    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_17e));
                    local_17e = ctx.s_1_1050_389a;
                    local_17c = &ctx.PTR_LOOP_1050_1008;
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
    let paVar1: *mut Struct493;
    let local_eax_22: *mut u8;
    let mut local_DX_22: u16;
    let mut local_DX_71: u16;
    let mut u_var2: i32;
    let mut local_SS__1: u16;
    let mut u_var3: u32;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u32;
    let mut local_a: u32;
    let mut local_6: u32;

    local_eax_22._0_2_ = &local_a;
    pass1_1030_64ce(
        _PTR_LOOP_1050_5740,
        param_1_00,
        param_2_00,
        local_eax_22,
        local_SS__1,
    );
    local_6 = local_eax_22;
    local_DX_71 = (local_eax_22 + 2);
    local_1e._3_1_ = (local_6 >> 0x18);
    if (local_1e._3_1_ == '\0') {
        return;
    }
    paVar1 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_6, local_DX_71);
    u_var3 = pass1_1030_73a8(CONCAT22(local_DX_71, paVar1));
    u_var2 = (u_var3 >> 0x10);
    if ((u_var2 | u_var3) != 0) {
        match (u_var3 + 0xc) {
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
    let mut i_var1: i32;
    let pu_var2: *mut u32;
    let ppc_var3: fn();
    let mut u_var4: u32;
    let local_AX_71: *mut Struct702;
    let local_AX_192: *mut Struct703;
    let pu_var5: *mut u32;
    let mut in_dx: u16;
    let mut extraout_dx: i32;
    let mut extraout_dx_00: i32;
    let mut u_var7: u16;
    let mut local_1c: u32;
    let mut local_14: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut u_var6: u32;

    u_var7 = (_PTR_LOOP_1050_4e74 >> 0x10);
    if ((_PTR_LOOP_1050_4e74 + param_1_00 * 6 + 4) == 0) {
        return;
    }
    if ((_PTR_LOOP_1050_4e74 + param_1_00 * 6 + 4) != -1) {
        if (PTR_LOOP_1050_4e78 == 0x0) {
            local_AX_71 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, 2, 0x400);
            pu_var2 = local_AX_71.field_0xc;
            unsafe { ppc_var3 = (*pu_var2 + 0x10) };
            pu_var5 = pu_var2;
            (**ppc_var3)();
            u_var4 = pu_var5 & 0xffff | extraout_dx << 0x10;
            local_14 = 0;
            while (local_14 < u_var4) {
                u_var6 = u_var4;
                pass1_1030_1d7c(pu_var2, local_14);
                if (((extraout_dx_00 | u_var6) != 0)
                    && ((i_var1 = (u_var6 + 0xc), i_var1 == 0x2a || (i_var1 == 0x2b))))
                {
                    PTR_LOOP_1050_4e78 = (&ctx.PTR_LOOP_1050_0000 + 1);
                    break;
                }
                local_14 = local_14 + 1;
            }
            if (PTR_LOOP_1050_4e78 == 0x0) {
                PTR_LOOP_1050_4e78 = (&ctx.PTR_LOOP_1050_0000 + 1);
                return;
            }
        }
        pass1_fn_1008_612e(0, (_PTR_LOOP_1050_4e74 + param_1_00 * 6 + 4) + -1);
    }
    return;
}

pub fn pass1_1020_b1ae(param_1: *mut u8, param_1_00: *mut u8, param_2_00: *mut u8) -> bool {
    let mut in_dx: u16;
    let mut unaff_ss: u16;
    let pu_var1: *mut u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut uStack8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_6 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, param_2_00, (param_2_00 >> 0x10));
    pu_var1 = pass1_1030_5b5c(local_6, in_dx);
    unsafe { local_c = *pu_var1 };
    uStack8 = (pu_var1 + 4);
    pass1_1008_3e94(
        param_1_00,
        CONCAT22(unaff_ss, &local_10),
        CONCAT22(unaff_ss, &local_e),
    );
    pass1_1008_3e94(
        &local_c,
        CONCAT22(unaff_ss, &local_14),
        CONCAT22(unaff_ss, &local_12),
    );
    if ((((0xb < local_e) && (0xb < local_10)) && (local_e < (local_12 - 0xb)))
        && (local_10 < (local_14 - 0xb)))
    {
        return 1;
    }
    return 0;
}

pub fn pass1_1020_b240(param_1: *mut u8, param_2: *mut u8, param_3: *mut u8) {
    let pu_var1: *mut u32;
    let paVar2: *mut Struct493;
    let BVar3: bool;
    let mut extraout_dx: u16;
    let mut u_var4: u16;
    let mut u_var5: i32;
    let mut u_var6: u16;
    let mut unaff_ss: u16;
    let mut u_var7: u32;
    let mut local_22: u32;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u32;
    let mut local_a: u32;
    let mut local_6: u32;

    pu_var1 = &local_a;
    u_var6 = (param_3 >> 0x10);
    pass1_1030_64ce(
        _PTR_LOOP_1050_5740,
        param_2,
        (param_3 + 4),
        pu_var1,
        unaff_ss,
    );
    unsafe { local_6 = *pu_var1 };
    u_var4 = (pu_var1 + 2);
    local_22._3_1_ = (local_6 >> 0x18);
    if (local_22._3_1_ != '\0') {
        paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_6, u_var4);
        u_var7 = pass1_1030_73a8(CONCAT22(u_var4, paVar2));
        u_var5 = (u_var7 >> 0x10);
        if (((u_var5 | u_var7) != 0) && (9 < (u_var7 + 0xc))) {
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
    let mut u_var1: u16;
    let mut u_var2: u16;
    let BVar3: bool;
    let mut i_var4: i32;
    let mut unaff_ss: u16;
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
        u_var2 = 0x4e6a;
    } else {
        u_var2 = 0x4e6e;
    }
    _local_c = CONCAT22(0x1050, u_var2);
    if (param_2 == 0) {
        local_14 = 0x4e68;
    } else {
        local_14 = 0x4e6c;
    }
    local_12 = &ctx.g_alloc_addr_1050_1050;
    _local_10 = CONCAT22(0x1050, local_14);
    loop {
        if (param_2 == 0) {
            ppwVar5 = &PTR_LOOP_1048_4230;
        } else {
            ppwVar5 = 0x10484236;
        }
        pass1_1008_3eb4(
            ppwVar5,
            CONCAT22(unaff_ss, &local_8),
            CONCAT22(unaff_ss, &local_6),
            CONCAT22(unaff_ss, &local_4),
        );
        u_var1 = *_local_c;
        if (u_var1 == 0) {
            _local_6 = CONCAT22(local_4 + *_local_10, local_6 - 1);
        } else {
            if (u_var1 == 1) {
                _local_6 = CONCAT22(local_4 - 1, local_6 + *_local_10);
            } else {
                if (u_var1 == 2) {
                    _local_6 = CONCAT22(local_4 - *_local_10, local_6 - 1);
                }
            }
        }
        pass1_1008_3e54(
            CONCAT22(unaff_ss, local_1a),
            local_8,
            _local_6,
            (_local_6 >> 0x10),
        );
        u_var2 = (param_4 >> 0x10);
        BVar3 = pass1_1020_b1ae(param_1, CONCAT22(unaff_ss, local_1a), *(param_4 + 4));
        if (BVar3 != 0) {
            i_var4 = pass1_1020_b240(param_1, CONCAT22(unaff_ss, local_1a), param_4);
            if (i_var4 != 0) {
                // LAB_1020_b46e:
                pass1_1008_3e76(param_3, local_8, _local_6, (_local_6 >> 0x10));
                return;
            }
        }
        u_var1 = *_local_c;
        if (u_var1 == 0) {
            // LAB_1020_b45e:
            _local_6 = _local_6 & 0xffff0000 | (local_6 + 2);
        } else {
            if (u_var1 == 1) {
                _local_6 = _local_6 & 0xffff | (local_4 + 2) << 0x10;
            } else {
                if (u_var1 == 2) {}
                // goto LAB_1020_b45e;
            }
        }
        pass1_1008_3e76(
            CONCAT22(unaff_ss, local_1a),
            local_8,
            _local_6,
            (_local_6 >> 0x10),
        );
        BVar3 = pass1_1020_b1ae(param_1, CONCAT22(unaff_ss, local_1a), *(param_4 + 4));
        if (BVar3 != 0) {
            i_var4 = pass1_1020_b240(param_1, CONCAT22(unaff_ss, local_1a), param_4);
            if (i_var4 != 0) {}
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
    let paVar2: *mut Struct493;
    let pu_var3: *mut u8;
    let pu_var4: *mut u16;
    let pu_var5: *mut u32;
    let mut extraout_dx: i32;
    let mut u_var6: i32;
    let mut unaff_ss: u16;
    let mut u_var7: u16;
    let mut u_var8: u16;
    // ppu_var9: *mut *mut u8;
    let pu_var10: *mut u8;
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

    pass1_1030_bcae(local_4, unaff_ss);
    pass1_1028_dc52(
        CONCAT22(unaff_ss, &local_16),
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        0x400,
    );
    while (true) {
        pu_var4 = &local_16;
        pass1_1028_e4ec(CONCAT22(unaff_ss, pu_var4));
        _local_1a = CONCAT22(extraout_dx, pu_var4);
        u_var6 = extraout_dx | pu_var4;
        if (u_var6 == 0) {
            pass1_1020_b240(param_1, param_2, param_3);
            if (pu_var4 != 0x0) {
                local_1e = (param_3 + 4);
                local_24 = param_2;
                uStack32 = (param_2 + 4);
                pass1_1008_3eb4(
                    CONCAT22(unaff_ss, &local_24),
                    CONCAT22(unaff_ss, &local_2a),
                    CONCAT22(unaff_ss, &local_28),
                    CONCAT22(unaff_ss, &local_26),
                );
                pass1_1008_3e76(
                    CONCAT22(unaff_ss, &local_24),
                    local_2a,
                    local_28 - 1,
                    local_26 - 1,
                );
                pu_var5 = &local_24;
                u_var7 = param_1;
                u_var8 = (param_1 >> 0x10);
                pass1_1020_afc4(u_var7, u_var8, CONCAT22(unaff_ss, pu_var5), local_1e);
                if (pu_var5 != 0x0) {
                    pass1_1008_3e76(
                        CONCAT22(unaff_ss, &local_24),
                        _local_2a,
                        (_local_2a >> 0x10),
                        local_26 - 1,
                    );
                    pu_var5 = &local_24;
                    pass1_1020_afc4(u_var7, u_var8, CONCAT22(unaff_ss, pu_var5), local_1e);
                    if (pu_var5 != 0x0) {
                        pass1_1008_3e76(
                            CONCAT22(unaff_ss, &local_24),
                            local_2a,
                            local_28 + 1,
                            local_26 - 1,
                        );
                        pu_var5 = &local_24;
                        pass1_1020_afc4(u_var7, u_var8, CONCAT22(unaff_ss, pu_var5), local_1e);
                        if (pu_var5 != 0x0) {
                            pass1_1008_3e76(
                                CONCAT22(unaff_ss, &local_24),
                                local_2a,
                                local_28 - 1,
                                local_26,
                            );
                            pu_var5 = &local_24;
                            pass1_1020_afc4(u_var7, u_var8, CONCAT22(unaff_ss, pu_var5), local_1e);
                            if (pu_var5 != 0x0) {
                                pass1_1008_3e76(
                                    CONCAT22(unaff_ss, &local_24),
                                    local_2a,
                                    local_28 + 1,
                                    local_26,
                                );
                                pu_var5 = &local_24;
                                pass1_1020_afc4(
                                    u_var7,
                                    u_var8,
                                    CONCAT22(unaff_ss, pu_var5),
                                    local_1e,
                                );
                                if (pu_var5 != 0x0) {
                                    pass1_1008_3e76(
                                        CONCAT22(unaff_ss, &local_24),
                                        local_2a,
                                        local_28 + 1,
                                        local_26 + 1,
                                    );
                                    pu_var5 = &local_24;
                                    pass1_1020_afc4(
                                        u_var7,
                                        u_var8,
                                        CONCAT22(unaff_ss, pu_var5),
                                        local_1e,
                                    );
                                    if (pu_var5 != 0x0) {
                                        pass1_1008_3e76(
                                            CONCAT22(unaff_ss, &local_24),
                                            _local_2a,
                                            (_local_2a >> 0x10),
                                            local_26 + 1,
                                        );
                                        pu_var5 = &local_24;
                                        pass1_1020_afc4(
                                            u_var7,
                                            u_var8,
                                            CONCAT22(unaff_ss, pu_var5),
                                            local_1e,
                                        );
                                        if (pu_var5 != 0x0) {
                                            pass1_1008_3e76(
                                                CONCAT22(unaff_ss, &local_24),
                                                local_2a,
                                                local_28 - 1,
                                                local_26 + 1,
                                            );
                                            pu_var5 = &local_24;
                                            pass1_1020_afc4(
                                                u_var7,
                                                u_var8,
                                                CONCAT22(unaff_ss, pu_var5),
                                                local_1e,
                                            );
                                            if (pu_var5 != 0x0) {
                                                pass1_1008_3e76(
                                                    CONCAT22(unaff_ss, &local_24),
                                                    local_2a,
                                                    local_28 - 2,
                                                    local_26 - 2,
                                                );
                                                pu_var5 = &local_24;
                                                pass1_1020_afc4(
                                                    u_var7,
                                                    u_var8,
                                                    CONCAT22(unaff_ss, pu_var5),
                                                    local_1e,
                                                );
                                                if (pu_var5 != 0x0) {
                                                    pass1_1008_3e76(
                                                        CONCAT22(unaff_ss, &local_24),
                                                        local_2a,
                                                        local_28 + 2,
                                                        local_26 - 2,
                                                    );
                                                    pu_var5 = &local_24;
                                                    pass1_1020_afc4(
                                                        u_var7,
                                                        u_var8,
                                                        CONCAT22(unaff_ss, pu_var5),
                                                        local_1e,
                                                    );
                                                    if (pu_var5 != 0x0) {
                                                        pass1_1008_3e76(
                                                            CONCAT22(unaff_ss, &local_24),
                                                            local_2a,
                                                            local_28 - 2,
                                                            local_26 + 2,
                                                        );
                                                        pu_var5 = &local_24;
                                                        pass1_1020_afc4(
                                                            u_var7,
                                                            u_var8,
                                                            CONCAT22(unaff_ss, pu_var5),
                                                            local_1e,
                                                        );
                                                        if (pu_var5 != 0x0) {
                                                            pass1_1008_3e76(
                                                                CONCAT22(unaff_ss, &local_24),
                                                                local_2a,
                                                                local_28 + 2,
                                                                local_26 + 2,
                                                            );
                                                            pu_var5 = &local_24;
                                                            pass1_1020_afc4(
                                                                u_var7,
                                                                u_var8,
                                                                CONCAT22(unaff_ss, pu_var5),
                                                                local_1e,
                                                            );
                                                            if (pu_var5 != 0x0) {
                                                                pass1_1008_3e76(
                                                                    CONCAT22(unaff_ss, &local_24),
                                                                    local_2a,
                                                                    local_28 - 1,
                                                                    local_26 + 2,
                                                                );
                                                                pu_var5 = &local_24;
                                                                pass1_1020_afc4(
                                                                    u_var7,
                                                                    u_var8,
                                                                    CONCAT22(unaff_ss, pu_var5),
                                                                    local_1e,
                                                                );
                                                                if (pu_var5 != 0x0) {
                                                                    pass1_1008_3e76(
                                                                        CONCAT22(
                                                                            unaff_ss, &local_24,
                                                                        ),
                                                                        local_2a,
                                                                        local_28 - 1,
                                                                        local_26 + 3,
                                                                    );
                                                                    pu_var5 = &local_24;
                                                                    pass1_1020_afc4(
                                                                        u_var7,
                                                                        u_var8,
                                                                        CONCAT22(unaff_ss, pu_var5),
                                                                        local_1e,
                                                                    );
                                                                    if (pu_var5 != 0x0) {
                                                                        local_2e = 3;
                                                                        while (true) {
                                                                            if (9 < local_2e) {
                                                                                return;
                                                                            }
                                                                            pass1_1008_3e76(
                                                                                CONCAT22(
                                                                                    unaff_ss,
                                                                                    &local_24,
                                                                                ),
                                                                                0,
                                                                                local_28 - local_2e,
                                                                                local_26,
                                                                            );
                                                                            pu_var5 = &local_24;
                                                                            pass1_1020_afc4(
                                                                                u_var7,
                                                                                u_var8,
                                                                                CONCAT22(
                                                                                    unaff_ss,
                                                                                    pu_var5,
                                                                                ),
                                                                                local_1e,
                                                                            );
                                                                            if (pu_var5 == 0x0) {
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
        u_var1 = (pu_var4 + 8);
        ppu_var9 = param_2;
        pu_var10 = param_3;
        paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
        pu_var3 = local_4;
        pass1_1030_bcbc(
            CONCAT22(unaff_ss, pu_var3),
            CONCAT22(u_var6, paVar2),
            ppu_var9,
            pu_var10,
        );
        if (pu_var3 < 0) {
            break;
        }
        if (pu_var3 < 0x65) {
            return;
        }
    }
    return;
}

pub fn call_infinite_loop_1020_b872(param_1: *mut u8, param_2: *mut u8) {
    let mut u_var1: u16;
    let mut u_var2: u16;
    let mut i_var3: i32;
    let mut unaff_ss: u16;
    let pu_var4: *mut u32;
    let u_var5: u8;
    let mut u_var6: u16;
    let mut local_13a: u16;
    let mut local_138: u16;
    let mut local_136: u16;
    let mut local_134: u16;
    let mut local_12: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let mut uStack4: u16;

    u_var6 = (param_2 >> 0x10);
    pu_var4 = pass1_1030_5b5c(param_2, u_var6);
    unsafe { local_8 = *pu_var4 };
    uStack4 = (pu_var4 + 4);
    u_var5 = (unaff_ss >> 8);
    pass1_1008_3e94(
        &local_8,
        CONCAT22(unaff_ss, &local_c),
        CONCAT22(unaff_ss, &local_a),
    );
    u_var1 = pass1_fn_1008_612e(10, local_a - 10);
    u_var2 = pass1_fn_1008_612e(10, local_c - 10);
    pass1_1008_3e54(
        CONCAT13(u_var5, CONCAT12(unaff_ss, &local_12)),
        0,
        u_var2,
        u_var1,
    );
    while (true) {
        i_var3 = infinite_loop_1020_b482(param_1, CONCAT22(unaff_ss, &local_12), param_2);
        if (i_var3 != 0) {
            break;
        }
        u_var1 = pass1_fn_1008_612e(10, local_a - 10);
        u_var2 = pass1_fn_1008_612e(10, local_c - 10);
        pass1_1008_3e76(
            CONCAT13(u_var5, CONCAT12(unaff_ss, &local_12)),
            0,
            u_var2,
            u_var1,
        );
    }
    pass1_1028_8888(
        CONCAT22(unaff_ss, &local_136),
        0,
        10,
        &local_12,
        unaff_ss,
        0x8000002,
        0,
        (param_2 + 4),
    );
    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_136));
    pass1_1020_b97e(param_1, (param_1 >> 0x10), 1);
    return;
}

pub fn pass1_1020_b97e(param_1: u16, param_2: u16, param_1_00: i32) {
    let paVar1: *mut Struct493;
    let mut in_dx: u16;
    let mut unaff_ss: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    paVar1 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, 2, 0x400);
    _PTR_LOOP_1050_4e70 = CONCAT22(in_dx, paVar1);
    local_6 = &paVar1.field_0x10;
    local_a = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_6, (local_6 >> 0x10));
    modify_list_1008_3f62(&PTR_LOOP_1048_4230, CONCAT22(in_dx, local_a + 0xc));
    pass1_1008_3e94(
        &PTR_LOOP_1050_4230,
        CONCAT22(unaff_ss, &local_e),
        CONCAT22(unaff_ss, &local_c),
    );
    if (param_1_00 == 0) {
        pass1_1008_3e76(&PTR_LOOP_1048_4230, 0, local_e + 1, local_c - 1);
        pass1_1008_3e94(
            &PTR_LOOP_1050_4230,
            CONCAT22(unaff_ss, &local_e),
            CONCAT22(unaff_ss, &local_c),
        );
    }
    pass1_1008_3e76(0x10484236, 1, local_e - 2, local_c);
    return;
}

pub fn pass1_1020_ba3e(param_1: *mut Struct704, param_2: u16, param_3: u16) {
    let local_bx_3: *mut Struct704;
    let local_es_3: *mut Struct704;

    local_es_3 = (param_1 >> 0x10);
    local_bx_3 = param_1;
    param_1 = 0;
    local_bx_3.field_0x4 = 0;
    local_bx_3.field_0x6 = param_3;
    local_bx_3.field_0x8 = param_2;
    if (local_bx_3.field_0x6 == 0) {
        local_bx_3.field_0x6 = 5;
    }
    call_alloc_mem_fn_1020_bcc4(param_1);
    return;
}

pub fn pass1_1020_ba7e(param_1: *mut *mut Struct44) {
    error_check_1000_17ce(param_1);
    return;
}

pub fn infinite_loop_1020_ba94(param_1: *mut long) {
    let pu_var1: *mut u32;
    let mut local_8: u16;
    let mut local_6: u32;

    let param_1_val = unsafe { *param_1 };
    if (param_1_val == 0) {
        return;
    }
    local_8 = 0;
    while (true) {
        pu_var1 = (param_1 + 4);
        let pu_var1_val = unsafe { *pu_var1 };
        if (pu_var1_val < local_8 || pu_var1_val == local_8) {
            break;
        }
        local_8 = local_8 + 1;
    }
    return;
}

pub fn pass1_1020_bae6(param_1: u16, param_2: u32) {
    let mut in_eax: u32;
    let local_DXAX_13: *mut u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1020_bc92(param_1, param_2);
    if ((local_DXAX_13._2_2_ | in_eax) != 0) {
        return in_eax & 0xffff0000 | *(in_eax & 0xffff | local_DXAX_13._2_2_ << 0x10);
    }
    return in_eax & 0xffff0000;
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
    let mut u_var1: u16;
    let mut b_var2: bool;
    let pu_var3: *mut u32;
    let mut local_6: u16;
    let mut local_4: u16;

    pu_var3 = pass1_1020_bc92(param_1, param_4._2_2_);
    u_var1 = (pu_var3 >> 0x10);
    if (pu_var3 == 0x0) {
        pu_var3 = pass1_1020_bc92(param_1, 0);
        if (pu_var3 == 0x0) {
            call_alloc_mem_fn_1020_bcc4(param_1);
            pu_var3 = pass1_1020_bc92(param_1, 0);
            if (pu_var3 == 0x0) {
                return 0;
            }
            (pu_var3 + 4) = param_4._2_2_;
        } else {
            (pu_var3 + 4) = param_4._2_2_;
        }
        u_var1 = (pu_var3 >> 0x10);
        if (param_2 != 0) {
            unsafe {
                b_var2 = CARRY2(*pu_var3, param_3);
                param_3 = *pu_var3 + param_3;
            }
            param_4._0_2_ = (pu_var3 + 2) + param_4 + b_var2;
        }
        unsafe {
            *pu_var3 = param_3;
            (pu_var3 + 2) = param_4
        };
        pass1_1020_bc72(param_1);
    } else {
        if (param_2 != 0) {
            unsafe {
                b_var2 = CARRY2(*pu_var3, param_3);
                param_3 = *pu_var3 + param_3;
            }
            param_4._0_2_ = (pu_var3 + 2) + param_4 + b_var2;
        }
        unsafe {
            *pu_var3 = param_3;
            (pu_var3 + 2) = param_4;
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
    let mut unaff_ss: u16;
    let mut local_c: [u8; 4];
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_8 = param_2;
    u_var1 = (param_1 + 2);
    let param_1_val = unsafe { *param_1 };
    pass1_1000_49c6(
        local_c,
        unaff_ss,
        param_1_val,
        u_var1,
        (u_var1 >> 0x10),
        6,
        0xbd6c,
    );
    return;
}

pub fn call_alloc_mem_fn_1020_bcc4(in_struct_1: *mut Struct705) {
    let pu_var1: *mut u32;
    let mut i_var2: i32;
    let mut u_var3: i32;
    let extraout_dx: *mut u16;
    let local_struct_1: *mut Struct705;
    let local_struct_1_hi: *mut Struct705;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    if (local_struct_1.field_0x4 == 0) {
        g_u16_ptr_1050_5f2e = 0x0;
        i_var2 = local_struct_1.field_0x6;
    } else {
        u_var3 = local_struct_1.field_0x4;
        pu_var1 = &local_struct_1.field_0x8;
        unsafe {
            i_var2 = u_var3 + *pu_var1;
            g_u16_ptr_1050_5f2e = CARRY2(u_var3, *pu_var1)
        };
    }
    if (g_u16_ptr_1050_5f2e == 0x0) {
        if (in_struct_1 == 0) {
            if (__g_Struct94_ptr_1 == 0) {
                struct_fn_1000_160a();
            } else {
            }
            u_var3 = i_var2 * 6;
            alloc_mem_1000_1708(u_var3, 0, 1);
        } else {
            u_var3 = i_var2 * 6;
            alloc_mem_1000_0ed4(1, u_var3, 0, in_struct_1);
            g_u16_ptr_1050_5f2e = extraout_dx;
        }
        _local_c = CONCAT22(g_u16_ptr_1050_5f2e, u_var3);
        if ((g_u16_ptr_1050_5f2e | u_var3) != 0) {
            local_struct_1.field_0x4 = i_var2;
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
        ctx._g_Struct372_1050_0ed0,
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
    let mut u_var1: u16;
    let mut unaff_ss: u16;
    let mut u_var2: u16;
    let mut in_stack_0000feba: u16;
    let mut local_136: u16;
    let mut local_134: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22(in_stack_0000feba, 0x2f),
    );
    local_c = (_local_6 >> 0x10);
    local_a = (_local_6 + 0x20);
    local_e = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_a, (local_a >> 0x10));
    if (param_2 != 0) {
        u_var1 = (param_2 >> 0x10);
        if ((param_2 + 1) == 0) {
            u_var2 = SUB42(&PTR_LOOP_1050_4230, 0);
        } else {
            u_var2 = SUB42(s_dib_1050_4234 + 2, 0);
        }
        pass1_1008_3f32(param_2, u_var2, &PTR_LOOP_1050_1048);
        pass1_1028_87f0(
            CONCAT22(unaff_ss, &local_136),
            0,
            0,
            param_3,
            param_2,
            u_var1,
            (_PTR_LOOP_1050_4e70 + 4),
            local_a,
        );
        pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_136));
        return;
    }
    pass1_1020_abc0(param_1, param_3, local_e, local_c);
    return;
}

pub fn pass1_1020_a54c(param_1: u16, param_2: u16, param_1_00: i32) {
    let mut u_var1: u32;
    let mut unaff_si: u16;
    let mut unaff_ss: u16;
    let mut u_var2: u16;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut u_var6: u16;
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

    _local_6 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x2f));
    local_c = (_local_6 >> 0x10);
    local_a = (_local_6 + 0x20);
    local_e = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_a, (local_a >> 0x10));
    local_14._0_4_ = _PTR_LOOP_1048_4230;
    local_14._4_2_ = PTR_LOOP_1048_4234;
    local_1c = local_14;
    pass1_1008_3e94(
        local_14,
        CONCAT22(unaff_ss, &local_18),
        CONCAT22(unaff_ss, &local_16),
    );
    if ((param_1_00 < 0) || (5 < param_1_00)) {
        pass1_1008_3e76(CONCAT22(unaff_ss, local_14), 0, local_18 - 9, local_16);
        u_var5 = local_a;
        u_var6 = (local_a >> 0x10);
        u_var1 = (_PTR_LOOP_1050_4e70 + 4);
        u_var3 = u_var1;
        u_var4 = (u_var1 >> 0x10);
        u_var2 = 0x14;
    } else {
        pass1_1008_3e76(
            CONCAT22(unaff_ss, local_14),
            0,
            (local_18 - param_1_00) - 3,
            local_16,
        );
        u_var5 = local_a;
        u_var6 = (local_a >> 0x10);
        u_var1 = (_PTR_LOOP_1050_4e70 + 4);
        u_var3 = u_var1;
        u_var4 = (u_var1 >> 0x10);
        u_var2 = 0x7b;
    }
    pass1_1028_87f0(
        CONCAT22(unaff_ss, &local_140),
        0,
        0,
        u_var2,
        local_14,
        unaff_ss,
        CONCAT22(u_var4, u_var3),
        CONCAT22(u_var6, u_var5),
    );
    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_140));
    return;
}

pub fn pass1_1020_8e6c(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1020_8bae(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_8eaa(param_1: *mut Struct393) {
    let mut u_var1: i32;
    let mut u_var2: u16;
    let mut u_var3: u16;
    let mut extraout_dx: u16;
    let local_bx_17: *mut Struct695;
    let mut unaff_si: u16;
    let mut u_var4: i32;
    let mut unaff_ss: u16;
    let ppVar5: *mut pass1_struct_1;

    process_struct_1020_847a(param_1, 0x25);
    u_var4 = (param_1 >> 0x10);
    local_bx_17 = param_1;
    &local_bx_17.field_0x16 = 0;
    local_bx_17.field_0xaa = 0;
    u_var1 = &local_bx_17.field_0xae;
    zero_list_1008_3e38((param_1 & 0xffff0000 | u_var1));
    &local_bx_17.field_0xb4 = 0;
    local_bx_17.field_0xb8 = 0xffff;
    &local_bx_17.field_0xba = 0;
    param_1.field_0x0 = 0x9204;
    local_bx_17.field_0x2 = 0x1020;
    ppVar5 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x29));
    u_var3 = (ppVar5 >> 0x10);
    u_var2 = ppVar5;
    local_bx_17.field_0x16 = u_var2;
    local_bx_17.field_0x18 = u_var3;
    pass1_1018_2646(
        local_bx_17.field_0x16,
        u_var3,
        param_1 & 0xffff0000 | u_var1,
    );
    mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, 0x1ce);
    local_bx_17.field_0xb4 = u_var2;
    local_bx_17.field_0xb6 = extraout_dx;
    pass1_1020_8712(
        (param_1 & 0xffff | u_var4 << 0x10),
        &stack0xfff6,
        unaff_ss,
        CONCAT22(extraout_dx, local_bx_17.field_0xb4),
        param_1 & 0xffff0000 | u_var1,
    );
    ppVar5 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 2));
    local_bx_17.field_0xba = ppVar5;
    local_bx_17.field_0xbc = (ppVar5 >> 0x10);
    return;
}

pub fn pass1_1020_8f74(param_1: *mut Struct44) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let local_bx_5: *mut Struct44;
    let local_es_5: *mut Struct44;
    let fn_ptr_1: fn();

    local_es_5 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    param_1.ptr_a_lo = 0x9204;
    local_bx_5.ptr_a_hi = 0x1020;
    pu_var1 = local_bx_5.field_0xb4;
    u_var2 = local_bx_5.field_0xb6;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            fn_ptr_1 = *pu_var1;
            (**fn_ptr_1)()
        };
    }
    pass1_1020_8556(param_1);
    return;
}

pub fn invalidate_rect_1020_8fb4(param_1: u32) {
    let mut i_var1: i32;
    let mut u_var2: u32;
    let mut in_AX: u16;
    let rect: *mut RECT16;
    let mut u_var3: u32;
    let mut extraout_dx: i32;
    let mut extraout_dx_00: i32;
    let mut hwnd: i32;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut local_16: u16;
    let mut local_e: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    u_var2 = (i_var4 + 0xba);
    if ((u_var2 + 0x1e) != 0) {
        pass1_1018_2862((i_var4 + 0x16));
        (i_var4 + 0xaa) = in_AX;
        (i_var4 + 0xac) = extraout_dx;
        if ((extraout_dx | (i_var4 + 0xaa)) != 0) {
            u_var2 = (i_var4 + 0xaa);
            i_var1 = (u_var2 + 10);
            local_8 = 0;
            while (local_8 < i_var1) {
                u_var3 = SEXT24(local_8);
                bad_func_1008_8fc4(*(i_var4 + 0xaa), u_var3);
                rect = u_var3;
                if ((((extraout_dx_00 | rect) != 0) && (9 < rect[5].bottom))
                    && (
                        pass1_1008_8b20((u_var3 & 0xffff | extraout_dx_00 << 0x10)),
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

pub fn pass1_1020_9068(in_struct_1: *mut Struct696) {
    let mut i_var1: i32;
    let mut u_var2: u32;
    let paVar3: *mut Struct318;
    let mut u_var4: u32;
    let mut u_var5: i32;
    let mut u_var6: u32;
    let mut extraout_dx: i32;
    let mut extraout_dx_00: u16;
    let mut extraout_dx_01: i32;
    let mut extraout_dx_02: u16;
    let local_struct_1: *mut Struct696;
    let mut i_var7: i32;
    let local_struct_1_hi: *mut Struct696;
    let mut u_var8: u16;
    let mut local_10: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let fn_ptr_1: fn();

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    paVar3 = local_struct_1.struct318_ptr_0x16;
    u_var2 = (paVar3 + 10);
    u_var6 = u_var2;
    pass1_1018_280c(local_struct_1.struct318_ptr_0x16);
    local_struct_1.field_0xaa = u_var6;
    &local_struct_1.field_0xac = extraout_dx;
    u_var5 = extraout_dx | local_struct_1.field_0xaa;
    if (u_var5 == 0) {
        pass1_1018_2862(local_struct_1.struct318_ptr_0x16);
        local_struct_1.field_0xaa = u_var5;
        &local_struct_1.field_0xac = extraout_dx_00;
    }
    if ((&local_struct_1.field_0xac | local_struct_1.field_0xaa) != 0) {
        pass1_1020_915a((in_struct_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10));
        pass1_1008_4480(
            u_var2,
            (in_struct_1 & 0xffff0000 | &local_struct_1.field_0xae),
            local_struct_1.field_0xb4,
        );
        fn_ptr_1 = (in_struct_1 + 0x10);
        (**fn_ptr_1)();
        u_var4 = &local_struct_1.field_0xaa;
        i_var1 = (u_var4 + 10);
        local_a = 0;
        while (local_a < i_var1) {
            u_var6 = SEXT24(local_a);
            bad_func_1008_8fc4(*&local_struct_1.field_0xaa, u_var6);
            u_var5 = u_var6;
            if ((extraout_dx_01 | u_var5) != 0) {
                pass1_1008_8c4e((u_var6 & 0xffff | extraout_dx_01 << 0x10), u_var2);
                u_var4 = local_struct_1.field_0xc;
                u_var8 = (u_var4 >> 0x10);
                i_var7 = u_var4;
                (i_var7 + local_a * 4) = u_var5;
                (i_var7 + local_a * 4 + 2) = extraout_dx_02;
            }
            local_a = local_a + 1;
        }
    }
    return;
}

pub fn pass1_1020_915a(param_1: *mut Struct697) {
    let mut i_var1: i32;
    let local_struct_1: *mut Struct697;
    let local_struct_1_hi: *mut Struct697;
    let ppVar2: *mut pass1_struct_1;
    let mut u_var3: u32;
    let mut local_c: u16;
    let pcStack14: *mut libc::c_char;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pcStack14 = CONCAT22(local_c, 0x2f);
    ppVar2 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, pcStack14);
    i_var1 = (ppVar2 + 0x1e);
    local_struct_1_hi = (param_1 >> 0x10);
    local_struct_1 = param_1;
    if (local_struct_1.field_0xb8 != i_var1) {
        local_c = 0x1ce;
        if (i_var1 == 1) {
            local_c = 0x1cf;
        } else {
            if (i_var1 == 2) {
                local_c = 0x1d0;
            } else {
                if (i_var1 == 3) {
                    local_c = 0x1d1;
                } else {
                    if (i_var1 == 4) {
                        local_c = 0x1d2;
                    }
                }
            }
        }
        u_var3 = mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, local_c);
        local_struct_1.field_0xb4 = u_var3;
        local_struct_1.field_0xb6 = (u_var3 >> 0x10);
        local_struct_1.field_0xb8 = i_var1;
    }
    return;
}

pub fn pass1_1020_91de(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1020_8f74(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn process_struct_1020_8296(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    process_struct_1020_808e(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn process_struct_1020_8360(in_struct_1: *mut Struct680) {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let mut u_var3: u16;
    let mut unaff_si: u16;
    let ppVar4: *mut pass1_struct_1;
    let local_struct_1: *mut Struct680;
    let local_struct_1_hi: *mut Struct680;
    let mut local_6: u32;

    local_struct_1 = in_struct_1;
    local_struct_1_hi = (in_struct_1 >> 0x10);
    process_struct_1020_847a(in_struct_1, 1);
    zero_list_1008_3e38((in_struct_1 & 0xffff0000 | &local_struct_1.field_0x16));
    &local_struct_1.field_0x1c = 0;
    in_struct_1.field_0x0 = 0x8462;
    local_struct_1.field_0x2 = 0x1020;
    ppVar4 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x29));
    u_var3 = (ppVar4 >> 0x10);
    local_struct_1.field_0x1c = ppVar4;
    &local_struct_1.field_0x1e = u_var3;
    pass1_1018_26f8(
        local_struct_1.field_0x1c,
        u_var3,
        in_struct_1 & 0xffff0000 | &local_struct_1.field_0x16,
    );
    u_var2 = &local_struct_1.field_0x1c;
    u_var1 = local_struct_1.field_0x8;
    pass1_1020_8712(
        (in_struct_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10),
        u_var1,
        (u_var1 >> 0x10),
        (u_var2 + 0x2a),
        in_struct_1 & 0xffff0000 | &local_struct_1.field_0x16,
    );
    return;
}

pub fn pass1_1020_83f8(param_1: *mut Struct417) {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let local_struct_1: *mut Struct417;
    let local_struct_1_hi: *mut Struct417;
    let mut local_6: u32;

    local_struct_1_hi = (param_1 >> 0x10);
    local_struct_1 = param_1;
    if (&local_struct_1.field_0x4 != 0) {
        u_var1 = &local_struct_1.field_0x1c;
        u_var2 = &local_struct_1.field_0x1c;
        pass1_1008_4480(
            (u_var1 + 10),
            (param_1 & 0xffff0000 | &local_struct_1.field_0x16),
            (u_var2 + 0x2a),
        );
    }
    return;
}

pub fn pass1_1020_843c(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1020_8556(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn process_struct_1020_847a(param_1: *mut Struct393, param_2: u16) {
    let mut u_var1: u16;
    let struct_a: *mut Struct199;
    let struct_a_00: *mut Struct199;
    let local_struct_1: *mut Struct393;
    let mut unaff_si: u16;
    let local_struct_1_hi: *mut Struct393;
    let ppVar2: *mut pass1_struct_1;

    local_struct_1_hi = (param_1 >> 0x10);
    local_struct_1._0_2_ = param_1;
    param_1.field_0x0 = ctx.s_1_1050_389a;
    (local_struct_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    (local_struct_1 + 4) = 0;
    (local_struct_1 + 6) = param_2;
    (local_struct_1 + 8) = 0;
    (local_struct_1 + 0xc) = 0;
    zero_list_1008_3e38((param_1 & 0xffff0000 | (local_struct_1 + 0x10)));
    param_1.field_0x0 = 0x87aa;
    (local_struct_1 + 2) = 0x1020;
    ppVar2 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x48));
    modify_list_1008_3f62(
        (param_1 & 0xffff0000 | (local_struct_1 + 0x10)),
        ppVar2 & 0xffff0000 | (ppVar2 + 0xe),
    );
    u_var1 = (local_struct_1 + 6) << 3;
    struct_a_00 = struct_a;
    process_struct_1000_179c(u_var1, struct_a);
    (local_struct_1 + 8) = u_var1;
    (local_struct_1 + 10) = struct_a_00;
    u_var1 = (local_struct_1 + 6) << 2;
    process_struct_1000_179c(u_var1, struct_a_00);
    (local_struct_1 + 0xc) = u_var1;
    (local_struct_1 + 0xe) = struct_a_00;
    pass1_1000_4906((local_struct_1 + 8), 0, (local_struct_1 + 6) << 3);
    pass1_1000_4906((local_struct_1 + 0xc), 0, (local_struct_1 + 6) << 2);
    return;
}

pub fn pass1_1020_8556(param_1: *mut Struct44) {
    let pu_var1: *mut u16;
    let mut u_var2: i32;
    let in_struct_1: *mut Struct44;
    let mut u_var3: u32;
    let local_bx_5: *mut Struct684;
    let mut i_var4: i32;
    let mut i_var5: i32;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut local_c: u16;
    let mut local_6: u32;
    let local_struct_1: *mut Struct44;

    u_var6 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    param_1.ptr_a_lo = 0x87aa;
    local_bx_5.field_0x2 = 0x1020;
    error_check_1000_17ce(local_bx_5.field_0x8);
    if ((&local_bx_5.field_0xe | local_bx_5.field_0xc) != 0) {
        local_c = 0;
        while (true) {
            pu_var1 = &local_bx_5.field_0x6;
            let pu_var1_val = unsafe { *pu_var1 };
            if (pu_var1_val == local_c || pu_var1_val < local_c) {
                break;
            }
            i_var5 = local_c * 4;
            u_var3 = &local_bx_5.field_0xc;
            u_var7 = (u_var3 >> 0x10);
            i_var4 = u_var3;
            if ((i_var4 + i_var5) != 0) {
                in_struct_1 = (i_var4 + i_var5);
                u_var2 = (i_var4 + i_var5 + 2);
                if ((u_var2 | in_struct_1) != 0) {
                    pass1_1008_5118((in_struct_1 & 0xffff | u_var2 << 0x10));
                    error_check_1000_17ce(in_struct_1);
                }
            }
            local_c = local_c + 1;
        }
        error_check_1000_17ce(&local_bx_5.field_0xc);
    }
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    local_bx_5.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub fn pass1_1020_85f6(param_1: *mut u8) {
    let pu_var1: *mut u16;
    let mut u_var2: i32;
    let in_struct_1: *mut Struct44;
    let mut u_var3: u32;
    let mut i_var4: i32;
    let local_bx_85: *mut u8;
    let mut u_var5: u16;
    let local_es_85: *mut u8;
    let mut local_8: u32;
    let mut local_4: u16;
    let temp_5f20445f9b: *mut Struct44;

    local_4 = 0;
    while (true) {
        local_es_85 = (param_1 >> 0x10);
        local_bx_85 = param_1;
        pu_var1 = (local_bx_85 + 6);
        let pu_var1_val = unsafe { *pu_var1 };
        if (pu_var1_val == local_4 || pu_var1_val < local_4) {
            break;
        }
        u_var3 = (local_bx_85 + 0xc);
        u_var5 = (u_var3 >> 0x10);
        i_var4 = u_var3;
        in_struct_1 = (i_var4 + local_4 * 4);
        u_var2 = (i_var4 + local_4 * 4 + 2);
        if ((u_var2 | in_struct_1) != 0) {
            pass1_1008_5118((in_struct_1 & 0xffff | u_var2 << 0x10));
            error_check_1000_17ce(in_struct_1);
        }
        u_var3 = (local_bx_85 + 0xc);
        (u_var3 + local_4 * 4) = 0;
        local_4 = local_4 + 1;
    }
    return;
}

pub fn pass1_1020_865a(param_1: *mut Struct681) {
    let pu_var1: *mut u16;
    let in_struct_1: *mut Struct44;
    let mut u_var2: u32;
    let local_bx_39: *mut Struct681;
    let local_bx_53: *mut Struct682;
    let mut i_var3: i32;
    let local_SI_50: *mut Struct683;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut local_8: u32;
    let mut local_4: u16;
    let local_struct_1: *mut Struct44;
    let local_struct_1_1: *mut Struct44;

    local_4 = 0;
    while (true) {
        u_var4 = (param_1 >> 0x10);
        local_bx_39 = param_1;
        pu_var1 = &local_bx_39.field_0x6;
        let pu_var1_val = unsafe { *pu_var1 };
        if (pu_var1_val == local_4 || pu_var1_val < local_4) {
            break;
        }
        local_SI_50 = (local_4 * 4);
        u_var2 = local_bx_39.field_0xc;
        u_var5 = (u_var2 >> 0x10);
        local_bx_53 = u_var2;
        if ((local_bx_53 + local_SI_50) != 0) {
            pass1_1008_5236((local_bx_53 + local_SI_50));
            u_var2 = local_bx_39.field_0xc;
            u_var5 = (u_var2 >> 0x10);
            i_var3 = u_var2;
            in_struct_1 = (local_SI_50 + i_var3);
            local_struct_1 = (local_SI_50 + i_var3 + 2);
            if ((local_struct_1 | in_struct_1) != 0) {
                pass1_1008_5118((in_struct_1 & 0xffff | ZEXT24(local_struct_1) << 0x10));
                error_check_1000_17ce(in_struct_1);
            }
            u_var2 = local_bx_39.field_0xc;
            (u_var2 + local_4 * 4) = 0;
        }
        local_4 = local_4 + 1;
    }
    return;
}

pub fn pass1_1020_86d8(param_1: *mut u8) {
    let pu_var1: *mut u16;
    let local_bx_17: *mut Struct685;
    let mut u_var2: u16;
    let mut local_4: u16;
    let mut temp_5f84f21f47: u32;

    local_4 = 0;
    while (true) {
        u_var2 = (param_1 >> 0x10);
        pu_var1 = (param_1 + 6);
        let pu_var1_val = unsafe { *pu_var1 };
        if (pu_var1_val == local_4 || pu_var1_val < local_4) {
            break;
        }
        temp_5f84f21f47 = (param_1 + 0xc);
        u_var2 = (temp_5f84f21f47 >> 0x10);
        local_bx_17 = temp_5f84f21f47;
        if ((local_bx_17 + local_4 * 4) != 0) {
            pass1_1008_5236((local_bx_17 + local_4 * 4));
        }
        local_4 = local_4 + 1;
    }
    return;
}

pub fn pass1_1020_8712(
    param_1: *mut Struct393,
    param_2: *mut Struct393,
    param_3: u16,
    param_4: u32,
    param_5: u32,
) -> i32 {
    let mut u_var1: u16;
    let mut u_var2: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1008_3f32(param_5, param_1 & 0xffff0000 | (param_1 + 0x10));
    u_var2 = process_struct_1008_4772(param_4);
    u_var1 = (u_var2 >> 0x10);
    pass1_1008_3e94(
        param_5,
        CONCAT22(param_3, &param_2.u16_0x2),
        CONCAT22(param_3, param_2),
    );
    param_2.u16_0x4 = (u_var2 + 4) + *_param_2;
    param_2.u16_0x6 = (u_var2 + 8) + param_2.u16_0x2;
    return;
}

pub fn pass1_1020_8784(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1020_8556(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_87c2(param_1: *mut Struct393) {
    let mut u_var1: u32;
    let local_AX_25: *mut Struct393;
    let mut unaff_si: u16;
    let mut u_var2: i32;
    let mut unaff_ss: u16;
    let ppVar3: *mut pass1_struct_1;
    let mut local_16: u32;
    let mut local_a: u16;
    let local_8: *mut Struct393;
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
    u_var2 = (param_1 >> 0x10);
    &local_AX_25.field_0x2e = 0;
    zero_list_1008_3e38((param_1 & 0xffff0000 | &local_AX_25.field_0x32));
    &local_AX_25.field_0x38 = 0;
    param_1.field_0x0 = 0x8a84;
    local_AX_25.u16_0x2 = 0x1020;
    ppVar3 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x29));
    &local_AX_25.field_0x2e = ppVar3;
    &local_AX_25.field_0x30 = (ppVar3 >> 0x10);
    local_a = 0;
    while {
        u_var1 = &local_AX_25.field_0x2e;
        pass1_1018_26d8(
            u_var1,
            (u_var1 >> 0x10),
            local_a,
            param_1 & 0xffff0000 | (&local_AX_25.field_0x14 + local_a * 6 + 2),
        );
        u_var1 = &local_AX_25.field_0x2e;
        pass1_1020_8712(
            (param_1 & 0xffff | u_var2 << 0x10),
            (&local_AX_25.field_0x8 + local_a * 8),
            &local_AX_25.field_0xa,
            (u_var1 + 0x2e + local_a * 4),
            param_1 & 0xffff0000 | (&local_AX_25.field_0x14 + local_a * 6 + 2),
        );
        local_a = local_a + 1;
        local_a < 4
    } {}
    u_var1 = &local_AX_25.field_0x2e;
    pass1_1018_2548(
        u_var1,
        (u_var1 >> 0x10),
        param_1 & 0xffff0000 | &local_AX_25.field_0x32,
    );
    u_var1 = &local_AX_25.field_0x2e;
    &local_AX_25.field_0x38 = (u_var1 + 0x6e);
    pass1_1020_8712(
        (param_1 & 0xffff | u_var2 << 0x10),
        &stack0xffee,
        unaff_ss,
        &local_AX_25.field_0x38,
        param_1 & 0xffff0000 | &local_AX_25.field_0x32,
    );
    return;
}

pub fn pass1_1020_8908(param_1: *mut Struct690, param_2: u32) {
    let in_struct_104_ptr: *mut Struct104;
    let mut u_var1: u32;
    let mut u_var2: i32;
    let local_DX_188: *mut Struct692;
    let paVar3: *mut Struct692;
    let mut extraout_dx: u16;
    let mut i_var4: i32;
    let local_bx_151: *mut Struct690;
    let local_bx_294: *mut Struct688;
    let mut i_var5: i32;
    let local_SI_209: *mut Struct689;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut u_var8: u32;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let local_4: *mut Struct687;
    let mut temp_7ffdca9fab4: i32;

    local_4 = 0x0;
    while (
        local_bx_151 = param_1,
        u_var6 = (param_1 >> 0x10),
        local_4 < 4,
    ) {
        if (local_bx_151.field_0x4 == 0) {
            u_var1 = local_bx_151.field_0xc;
            u_var6 = (u_var1 >> 0x10);
            i_var4 = u_var1;
            i_var5 = local_4 * 4;
            if (((i_var4 + i_var5 + 2) | (i_var4 + i_var5)) != 0) {
                pass1_1008_5236((i_var4 + i_var5));
            }
        } else {
            u_var1 = local_bx_151.field_0x2e;
            in_struct_104_ptr = (u_var1 + 0x2e + local_4 * 4);
            u_var8 = process_struct_1008_4772(in_struct_104_ptr);
            local_DX_188 = (u_var8 >> 0x10);
            temp_7ffdca9fab4 = u_var8;
            u_var1 = local_bx_151.field_0xc;
            local_SI_209 = (local_4 * 4);
            if ((&local_SI_209.field_0x0 + u_var1) == 0) {
                paVar3 = local_DX_188;
                u_var2 = temp_7ffdca9fab4;
                process_struct_1000_179c(0x14, local_DX_188);
                _local_1c = CONCAT22(paVar3, u_var2);
                if ((paVar3 | u_var2) == 0) {
                    u_var1 = local_bx_151.field_0xc;
                    (u_var1 + local_4 * 4) = 0;
                } else {
                    u_var2 = &local_bx_151.field_0x16 + local_4 * 6;
                    process_struct_1008_50c2(
                        _local_1c,
                        (temp_7ffdca9fab4 + 8),
                        (temp_7ffdca9fab4 + 4),
                        param_1 & 0xffff0000 | u_var2,
                        param_2,
                    );
                    u_var1 = local_bx_151.field_0xc;
                    u_var7 = (u_var1 >> 0x10);
                    local_bx_294 = u_var1;
                    (local_bx_294 + local_SI_209) = u_var2;
                    (local_bx_294 + local_SI_209 + 2) = extraout_dx;
                }
                u_var1 = local_bx_151.field_0xc;
                pass1_1008_5134((u_var1 + local_4 * 4));
            }
            u_var1 = local_bx_151.field_0xc;
            pass1_1008_5236((u_var1 + local_4 * 4));
            pass1_1008_4480(
                param_2,
                (param_1 & 0xffff0000 | (&local_bx_151.field_0x16 + local_4 * 6)),
                in_struct_104_ptr,
            );
        }
        local_4 = &local_4.field_0x1;
    }
    if (local_bx_151.field_0x4 != 0) {
        pass1_1008_4480(
            param_2,
            (param_1 & 0xffff0000 | &local_bx_151.field_0x32),
            local_bx_151.field_0x38,
        );
    }
    return;
}

pub fn pass1_1020_8a5e(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1020_8556(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_8bae(param_1: *mut Struct44) {
    param_1.ptr_a_lo = 0x8e92;
    (param_1 + 2) = 0x1020;
    pass1_1020_8556(param_1);
    return;
}

pub fn pass1_1020_6498(param_1: *mut u8, param_2: u16) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut u_var3: u16;

    u_var3 = (param_1 >> 0x10);
    if ((param_1 + 0x18 + param_2 * 4) != 0) {
        u_var1 = (param_1 + 0x18 + param_2 * 4);
        u_var3 = (u_var1 >> 0x10);
        i_var2 = u_var1;
        return CONCAT22((i_var2 + 10), (i_var2 + 8));
    }
    return 0;
}

pub fn pass1_1020_64d4(param_1: *mut u8, param_2: u16) {
    let mut local_es_5: u16;
    let mut temp_5ff6edc30e: u32;

    local_es_5 = (param_1 >> 0x10);
    if ((param_1 + 0x18 + param_2 * 4) != 0) {
        temp_5ff6edc30e = (param_1 + 0x18 + param_2 * 4);
        return (temp_5ff6edc30e + 4);
    }
    return 0;
}

pub fn pass1_1020_61c4(param_1: u16, param_2: u16, param_1_00: u32, param_2_00: *mut u16) {
    let mut u_var1: u16;
    let local_AX__1: *mut Struct667;
    let mut local_DX_11: u16;
    let ppVar2: *mut pass1_struct_1;
    let local_string_1: *mut libc::c_char;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut temp_5f50cbac33: u32;

    ppVar2 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_string_1, 0x2f));
    u_var1 = (ppVar2 >> 0x10);
    pass1_1030_8308(
        ctx._g_bool_1050_5748,
        (ctx._g_bool_1050_5748 >> 0x10),
        param_1_00,
        param_2_00,
        (ppVar2 + 0x20),
    );
    unsafe { *param_2_00 = (ppVar2 + 0x1e) };
    return;
}

pub fn pass1_1020_5d56(param_1: *mut u32, param_2: u32) -> bool {
    let pp_var1: fn();
    let mut unaff_ss: u16;
    let mut u_var2: u16;
    let mut u_var3: u16;
    let mut local_12: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_6 = (param_2 + 0x2e);
    u_var2 = param_1;
    u_var3 = (param_1 >> 0x10);
    if (local_6 == 0x47) {
        pass1_1020_61c4(
            u_var2,
            u_var3,
            CONCAT22(unaff_ss, &local_c),
            CONCAT22(unaff_ss, &local_a),
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
            u_var2,
            u_var3,
            CONCAT22(unaff_ss, &local_e),
            CONCAT22(unaff_ss, &local_12),
        );
        if (local_e <= local_12) {
            // LAB_1020_5d8b:
            unsafe {
                pp_var1 = (*param_1 + 0x40);
                (**pp_var1)();
            }
            return 1;
        }
    }
    pass1_1038_af40(ctx._g_Struct112_a, *(u_var2 + 8), 9);
    return 1;
}

pub fn call_draw_fn_1020_3bd6(in_struct_1: *mut Struct657) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let local_struct_1: *mut Struct657;
    let local_struct_1_hi: *mut Struct657;
    let pu_var3: *mut u8;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    draw_1020_3fa0(local_struct_1.field_0xf6);
    if (local_struct_1.field_0x100 == 0) {
        local_struct_1.field_0x100 = 1;
        u_var1 = local_struct_1.field_0xfa;
        if ((u_var1 + 0x56) == 0) {
            u_var2 = 5;
        } else {
            u_var2 = 8;
        }
        pu_var3 = pass1_1038_af40(ctx._g_Struct112_a, local_struct_1.field_0x8, u_var2);
        local_struct_1.field_0x10e = pu_var3;
        local_struct_1.field_0x110 = (pu_var3 >> 0x10);
    }
    return;
}

pub fn pass1_1020_3c32(param_1: *mut Struct658, param_2: u16, uparam_2_00: i32) {
    let mut u_var1: u32;
    let mut c_var2: u8;
    let mut u_var3: u16;

    if (param_2_00 == 0xf5) {
        u_var3 = 1;
        // LAB_1020_3c52:
        u_var1 = param_1.field_0xfa;
        pass1_1018_1b02(u_var1, (u_var1 >> 0x10), u_var3);
        return;
    }
    if ((param_2_00 < 0xf6) && (c_var2 = param_2_00, c_var2 != '\0')) {
        if (c_var2 == 0x1 || c_var2 == 0x2) {
            return;
        }
        if (c_var2 == -0xc) {
            u_var3 = 0;
            // goto LAB_1020_3c52;
        }
    }
    pass1_1020_3c32(param_1, param_2, param_2_00);
    return;
}

pub fn pass1_1020_3540(param_1: u16, param_2: u16, param_1_00: i32, param_2_00: u32) {
    let local_struct_2: *mut Struct655;
    let struct_a: *mut Struct199;
    let paVar1: *mut Struct199;
    let local_struct_1: *mut Struct654;
    let mut unaff_ss: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1008_3e94(
        param_2_00,
        CONCAT22(unaff_ss, &local_6),
        CONCAT22(unaff_ss, &local_4),
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

pub fn pass1_1020_2a94(param_1: &mut Vec<u8>, param_2: u32) {
    pass1_1018_1662((param_1 + 0xf2), param_2);
    return;
}

pub fn pass1_1020_2936(param_1: u16, param_2: u32) -> u8 {
    let mut u_var1: u16;

    u_var1 = return_1_1020_79ae();
    return u_var1;
}

pub fn pass1_1020_294a(in_struct_1: *mut Struct651, param_2: u32, param_3: u16) {
    let mut u_var1: u16;
    let local_struct_1: *mut Struct651;
    let mut unaff_BP: u16;
    let local_struct_1_hi: *mut Struct651;
    let ppVar2: *mut pass1_struct_1;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    local_struct_1.field_0xfc = param_3;
    ppVar2 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_BP, param_3));
    u_var1 = (ppVar2 >> 0x10);
    local_struct_1.field_0xf2 = ppVar2;
    &local_struct_1.field_0xf4 = u_var1;
    local_struct_1.field_0xe0 = local_struct_1.field_0xf2;
    local_struct_1.field_0xe2 = u_var1;
    pass1_1018_0902(&local_struct_1.field_0xf2, param_2);
    return;
}

pub fn process_struct_1020_26e6(param_1: *mut Struct376, param_2: u8) -> *mut Struct376 {
    process_struct_1020_2594(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}
