pub fn modify_list_1008_5b6e(ctx: &mut AppContext, param_1: *mut u16, param_2: u8) -> *mut u16 {
    unsafe { *param_1 = ctx.s_1_1050_389a };
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        free_mem_1000_093a(param_1);
    }
    return param_1;
}

pub fn modify_u16_list_1008_5bdc(in_struct_a: *mut Struct375) {
    let mut local_AX_17: u16;
    let mut local_DX_63: u16;
    let struct_a: *mut Struct375;
    let mut local_BP__1: u16;
    let struct_a_hi: *mut Struct375;
    let ppVar1: *mut pass1_struct_1;

    process_struct_1010_1d48(in_struct_a, 0x44);
    struct_a_hi = (in_struct_a >> 0x10);
    struct_a = in_struct_a;
    struct_a.ptr_2_lo = 0;
    &struct_a.ptr_2_hi = 0;
    struct_a.field_0x10 = 0;
    &struct_a.u32_x12 = 0;
    in_struct_a.ptr_1_lo = 0x5fc8;
    struct_a.ptr_1_hi = &ctx.PTR_LOOP_1050_1008;
    ctx._g_struct_ptr_1050_02a0 = in_struct_a;
    ppVar1 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_BP__1, 2));
    local_DX_63 = (ppVar1 >> 0x10);
    struct_a.ptr_2_hi = ppVar1;
    struct_a.u32_x0e = local_DX_63;
    return;
}

pub fn modify_u16_list_1008_5c34(in_u16_list: *mut u16) {
    unsafe { *in_u16_list = 0x5fc8 };
    (in_u16_list + 2) = &ctx.PTR_LOOP_1050_1008;
    ctx._g_struct_ptr_1050_02a0 = 0;
    pass1_1010_1d80(in_u16_list);
    return;
}

pub fn zero_list_1008_6c90(param_1: *mut Struct1) {
    zero_list_1008_3e38(param_1);
    zero_list_1008_3e38((param_1 & 0xffff0000 | (param_1 + 6)));
    return;
}

pub fn modify_list_1008_6cb4(
    param_1: *mut u32,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
) -> *mut u32 {
    let mut iVar1: i32;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    iVar1 = param_1;
    unsafe {
        *param_1 = param_4;
    }
    (iVar1 + 4) = (param_4 + 4);
    (iVar1 + 6) = param_2;
    (iVar1 + 10) = (param_2 + 4);
    return param_1;
}

pub fn modify_list_1008_6d18(param_1: *mut u16, param_2: u32, param_3: u32) {
    modify_list_1008_3f62(param_1, param_3);
    modify_list_1008_3f62((param_1 & 0xffff0000 | (param_1 + 6)), param_2);
    return;
}

pub fn modify_list_1008_6d3e(param_1: u32, param_2: *mut u16, param_3: *mut u16) {
    modify_list_1008_3f62(param_3, param_1);
    modify_list_1008_3f62(param_2, param_1 & 0xffff0000 | (param_1 + 6));
    return;
}

pub fn modify_list_1008_6d64(param_1: u32, param_2: *mut u16) {
    modify_list_1008_3f62(param_2, param_1);
    pass1_1008_3ee2(param_2, param_1 & 0xffff0000 | (param_1 + 6));
    return;
}

pub fn set_array_val_1008_72a8(param_1: *mut u16, param_2: u16) -> *mut u16 {
    unsafe { *param_1 = param_2 };
    return param_1;
}

pub fn clear_list_elements_1008_80d2(in_u32: *mut u32) -> *mut u32 {
    unsafe { *in_u32 = 0 };
    (in_u32 + 4) = 0;
    return in_u32;
}

pub fn modify_list_1008_8168(in_list: *mut u16) {
    let mut local_es_4: u16;

    local_es_4 = (in_list >> 0x10);
    unsafe {
        *in_list = 0x87c8;
        (in_list + 2) = &ctx.PTR_LOOP_1050_1008;
        *in_list = ctx.s_1_1050_389a;
        (in_list + 2) = &ctx.PTR_LOOP_1050_1008;
    }
    return;
}

pub fn modify_list_1008_87a2(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    modify_list_1008_8168(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_87cc(
    param_1: *mut Struct204,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u32,
    param_6: u32,
) {
    let mut local_AX_174: u16;
    let mut u_var1: i32;
    let mut u_var2: u16;
    let mut local_AX_507: u16;
    let mut i_var3: i32;
    let mut local_DX_174: u16;
    let mut local_DX_196: u16;
    let struct_a: *mut Struct199;
    let pa_var5: *mut Struct199;
    let mut local_DX_401: u16;
    let mut local_DX_507: u16;
    let struct_204_1: *mut Struct204;
    let local_bx_223: *mut Struct205;
    let mut unaff_si: u16;
    let mut local_es_6: u16;
    let mut local_es_223: u16;
    let mut local_es_653: u16;

    let puVar6: *mut u16;
    let mut local_34: u32;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_24: u32;
    let mut uStack32: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut temp_5fa25672bf: u32;
    let mut temp_7ffcbc94a91: u16;
    let mut u_var4: u32;

    local_es_6 = (param_1 >> 0x10);
    struct_204_1 = param_1;
    param_1 = ctx.s_1_1050_389a;
    struct_204_1.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    struct_204_1.field_0x4 = param_5;
    &struct_204_1.field_0x8 = 0;
    struct_204_1.field_0xc = param_3;
    struct_204_1.field_0xe = param_2;
    struct_204_1.field_0x10 = 0;
    struct_204_1.field_0x12 = 0;
    zero_list_1008_3e38((param_1 & 0xffff0000 | ZEXT24(&struct_204_1.field_0x1c)));
    zero_list_1008_3e38((param_1 & 0xffff0000 | &struct_204_1.field_0x22));
    zero_list_1008_3e38((param_1 & 0xffff0000 | &struct_204_1.field_0x28));
    struct_204_1.field_0x2e = param_4;
    struct_204_1.field_0x30 = 0xffff;
    struct_204_1.field_0x3a = 0;
    struct_204_1.field_0x3e = 1;
    struct_204_1.field_0x40 = 1;
    struct_204_1.field_0x42 = param_6;
    param_1 = 0x8e9a;
    struct_204_1.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    if (_PTR_LOOP_1050_0382 == 0x0) {
        _PTR_LOOP_1050_0382 =
            process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x2e));
    }
    _local_6 = process_struct_1008_4772(struct_204_1.field_0x4);
    local_DX_196 = (_local_6 >> 0x10);
    struct_204_1.field_0x12 = 0x2f - (_local_6 + 8);
    local_es_223 = (_PTR_LOOP_1050_0382 >> 0x10);
    local_bx_223 = _PTR_LOOP_1050_0382;
    local_8 = local_bx_223.field_0xa;
    local_a = local_bx_223.field_0xc;
    local_c = local_bx_223.field_0xe;
    local_e = local_bx_223.field_0x10;
    i_var3 = struct_204_1.field_0xc;
    pass1_1008_3e76(
        (param_1 & 0xff000000 | CONCAT12((param_1 >> 0x10), &struct_204_1.field_0x1c)),
        0,
        (i_var3 + struct_204_1.field_0xe) * local_e + struct_204_1.field_0x12 + local_a,
        (i_var3 - struct_204_1.field_0xe) * local_c + struct_204_1.field_0x10 + local_8,
    );
    struct_204_1.field_0x14 = struct_204_1.field_0x1c + 0x20;
    struct_204_1.field_0x16 = (_local_6 + 8) + &struct_204_1.field_0x1e + -0x25;
    struct_204_1.field_0x18 = struct_204_1.field_0x14 + 0x32;
    u_var1 = struct_204_1.field_0x16 + 0x19;
    struct_204_1.field_0x1a = u_var1;
    pa_var5 = struct_a;
    process_struct_1000_179c(6, struct_a);
    _local_1a = CONCAT22(pa_var5, u_var1);
    if ((pa_var5 | u_var1) == 0) {
        &struct_204_1.field_0x8 = 0;
    } else {
        puVar6 = pass1_1008_ada2(CONCAT22(pa_var5, u_var1), struct_204_1.field_0x2e);
        local_DX_401 = (puVar6 >> 0x10);
        struct_204_1.field_0x8 = puVar6;
        &struct_204_1.field_0xa = local_DX_401;
    }
    u_var2 = pass1_1008_aed8(&struct_204_1.field_0x8);
    if (u_var2 == 0) {
        _local_1a = &struct_204_1.field_0x8;
        local_12 = _local_1a;
        error_check_1000_17ce(_local_1a);
        &struct_204_1.field_0x8 = 0;
    } else {
        u_var4 = &struct_204_1.field_0x8;
        pass1_1018_20ee(_PTR_LOOP_1050_0382, u_var4);
        local_12 = u_var4 & 0xffff | local_DX_507 << 0x10;
        pass1_1008_add2(&struct_204_1.field_0x8);
        _local_1e = process_struct_1008_4772(local_12);
        pass1_1018_214e(
            _PTR_LOOP_1050_0382,
            (_PTR_LOOP_1050_0382 >> 0x10),
            param_1 & 0xffff0000 | &struct_204_1.field_0x28,
            struct_204_1.field_0x2e,
        );
        local_24 = &struct_204_1.field_0x1c;
        uStack32 = &struct_204_1.field_0x20;
        pass1_1008_3f32(
            0xdc,
            ctx.stack_seg_reg,
            &struct_204_1.field_0x28,
            local_es_6,
        );
        _local_30 = (param_1 & 0xffff0000 | &struct_204_1.field_0x32);
        pass1_1008_3e94(
            &local_24,
            (param_1 & 0xffff0000 | ZEXT24(&struct_204_1.field_0x34)),
            (param_1 & 0xffff0000 | &struct_204_1.field_0x32),
        );
        temp_7ffcbc94a91 = (_local_1e >> 0x10);
        struct_204_1.field_0x36 = (_local_1e + 4) + *_local_30;
        i_var3 = (_local_1e + 8) + struct_204_1.field_0x34;
        struct_204_1.field_0x38 = i_var3;
        pass1_fn_1008_612e(2, 5);
        struct_204_1.field_0x3e = i_var3;
    }
    return;
}

pub fn modify_list_1010_2b50(param_1: *mut u16, param_2: u16, param_1_00: *mut u16) {
    modify_list_1008_3f62(param_1_00, &PTR_LOOP_1048_0000);
    return;
}
