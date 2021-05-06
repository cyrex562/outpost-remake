use crate::{
    cleanup::win_cleanup_func_1040_782c,
    defines::{AppContext, Struct44, Struct444},
    list_funcs::modify_u16_list_1008_5c34,
    other_funcs::return_1008_53aa,
    pass4_funcs::pass1_1028_e2e0,
    pass5_funcs::{pass1_1030_8128, pass1_1030_8210, pass1_1030_8334, pass1_1030_835a},
    pass8_funcs::{pass1_1010_2050, pass1_1010_7efc},
    pass_funcs::{pass1_1008_397a, pass1_1008_8aa2, pass1_fn_1000_21b6},
    string_funcs::{
        copy_string_1000_3d3e, fn_1008_6048, pass1_1030_532e, process_string_1000_3cea,
        process_string_1000_4d58,
    },
    struct_funcs::process_struct_1000_179c,
    sys_funcs::{dos3_call_1000_51aa, pass1_1030_838e},
    ui_funcs::{msg_box_1000_1f24, pass1_1038_af34, win_cleanup_fn_1040_a294},
    util::{CONCAT22, SUB42},
};

pub unsafe fn error_check_1000_0dc6(ctx: &mut AppContext, param_1: u16, param_2: u16) -> bool {
    let mut unaff_cs: u16;
    let mut local_8: u16;

    if ((&ctx.PTR_LOOP_1050_000c & 0xfff8) != 0xcad0) {
        invoke_error_handler_1000_1e61(unaff_cs, 0xe, 0);
        return 0;
    }
    (**&ctx.BYTE_1050_0008)(&ctx.g_alloc_addr_1050_1050);
    return 1;
}

pub unsafe fn error_check_1000_16ee(ctx: &mut AppContext, param_1: i32, param_2: i32) {
    if ((param_2 | param_1) != 0) {
        error_check_1000_0dc6(ctx, param_1, param_2);
    }
    return;
}

pub unsafe fn error_check_1000_17ce(in_struct_1: *mut Struct44) -> u8 {
    let mut bVar1: u8;
    let BVar2: bool;

    bVar1 = (in_struct_1 >> 0x10) | in_struct_1;
    if (in_struct_1 != 0x0) {
        BVar2 = error_check_1000_0dc6(in_struct_1, in_struct_1._2_2_);
        bVar1 = BVar2;
    }
    return bVar1;
}

pub unsafe fn error_check_1000_18d2(ctx: &mut AppContext, param_1: i32, param_2: i32) {
    if ((param_2 | param_1) != 0) {
        error_check_1000_0dc6(ctx, param_1, param_2);
    }
    return;
}

pub unsafe fn invoke_error_handler_1000_1e61(ctx: &mut AppContext) {
    _SHI_INVOKEERRORHANDLER1(ctx);
    return;
}

pub unsafe fn _SHI_INVOKEERRORHANDLER1(ctx: &mut AppContext) -> u16 {
    let mut u_var1: i32;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let mut local_40: u16;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var4 = SUB42(&ctx.g_alloc_addr_1050_1050, 0);
    local_4 = &ctx.g_alloc_addr_1050_1050;
    if ((ctx.PTR_LOOP_1050_5f1c | ctx.PTR_s__1050_1f7e_1050_5f1a) == 0) {
        local_6 = 0x0;
        local_4 = 0x0;
    } else {
        u_var1 = pass1_fn_1000_21b6(ctx.PTR_s__1050_1f7e_1050_5f1a, ctx.PTR_LOOP_1050_5f1c);
        local_6 = ctx.PTR_s__1050_1f7e_1050_5f1a;
        local_4 = ctx.PTR_LOOP_1050_5f1c;
        if (u_var1 == 0) {
            ctx.PTR_s__1050_1f7e_1050_5f1a = (ctx.s_554_bmp_1050_1f77 + 7);
            ctx.PTR_LOOP_1050_5f1c = &ctx.PTR_LOOP_1050_1000;
            local_6 = (ctx.s_554_bmp_1050_1f77 + 7);
            local_4 = &ctx.PTR_LOOP_1050_1000;
        }
    }
    if ((local_4 | local_6) != 0) {
        i_var2 = msg_box_1000_1f24(&ctx.PTR_s__1050_1f7e_1050_5f1a, &ctx.g_alloc_addr_1050_1050);
        if (i_var2 == 0) {
            u_var3 = (*local_6)();
        } else {
            local_4 = 0;
            local_6 = 0;
            u_var3 = 0;
        }
        if ((local_4 | local_6) != 0) {
            check_and_clear_global_1000_1f68(u_var4);
        }
        return u_var3;
    }
    return 0;
}

pub unsafe fn handle_error_1008_0036(ctx: &mut AppContext, param_1: *mut u16) {
    let mut u_var1: i32;
    let pu_var2: *mut u32;
    let paVar3: *mut Struct44;
    let in_struct_1: *mut Struct444;
    let mut i_var4: i32;
    let mut local_es_4: u16;
    let mut local_CS__1: u16;
    let mut local_a: u32;
    let mut local_6: u32;
    let mut fn_ptr_1: Vec<u8>;
    let mut fn_ptr_2: Vec<u8>;
    let mut temp_5f1cd1a162: u32;
    let temp_862db56c250: *mut u32;
    let mut fn_ptr_3: Vec<u8>;

    local_es_4 = (param_1 >> 0x10);
    i_var4 = param_1;
    unsafe { *param_1 = 0x51e };
    (i_var4 + 2) = &ctx.PTR_LOOP_1050_1008;
    temp_5f1cd1a162 = (i_var4 + 8);
    u_var1 = (i_var4 + 10);
    if ((u_var1 | temp_5f1cd1a162) != 0) {
        return_1008_53aa(temp_5f1cd1a162, u_var1);
        local_CS__1 = 0x1000;
        error_check_1000_17ce(temp_5f1cd1a162);
    }
    paVar3 = ctx._g_bool_1050_5748;
    ctx._PTR_LOOP_1050_0298 = 0;
    if (ctx._g_bool_1050_5748 != 0x0) {
        pass1_1030_8210(ctx._g_bool_1050_5748, (ctx._g_bool_1050_5748 >> 0x10));
        local_CS__1 = 0x1000;
        error_check_1000_17ce(paVar3);
    }
    paVar3 = ctx._g_Struct372_1050_0ed0;
    if (ctx._g_Struct372_1050_0ed0 != 0x0) {
        pass1_1010_2050(
            ctx._g_Struct372_1050_0ed0,
            (ctx._g_Struct372_1050_0ed0 >> 0x10),
        );
        local_CS__1 = 0x1000;
        error_check_1000_17ce(paVar3);
    }
    in_struct_1 = ctx._g_struct_73_1050_14cc;
    if (ctx._g_struct_73_1050_14cc != 0x0) {
        pass1_1010_7efc(ctx._g_struct_73_1050_14cc);
        local_CS__1 = 0x1000;
        error_check_1000_17ce(in_struct_1);
    }
    paVar3 = ctx._g_Struct112_a;
    if (ctx._g_Struct112_a != 0x0) {
        pass1_1038_af34(ctx._g_Struct112_a, (ctx._g_Struct112_a >> 0x10));
        local_CS__1 = 0x1000;
        error_check_1000_17ce(paVar3);
    }
    if (ctx._PTR_LOOP_1050_1040 != 0x0) {
        fn_ptr_1 = ctx.u16_PTR_LOOP_1050_5bc8;
        (**fn_ptr_1)(
            local_CS__1,
            ctx._PTR_LOOP_1050_1040,
            (ctx._PTR_LOOP_1050_1040 >> 0x10),
            1,
        );
    }
    if (ctx._g_struct_ptr_1050_02a0 != 0x0) {
        fn_ptr_2 = ctx._g_struct_ptr_1050_02a0;
        (**fn_ptr_2)(
            local_CS__1,
            ctx._g_struct_ptr_1050_02a0,
            (ctx._g_struct_ptr_1050_02a0 >> 0x10),
            1,
        );
    }
    pu_var2 = (i_var4 + 4);
    u_var1 = (i_var4 + 6);
    if ((u_var1 | pu_var2) != 0) {
        fn_ptr_3 = pu_var2;
        (**fn_ptr_3)(local_CS__1, pu_var2, u_var1, 1);
    }
    handle_error_1008_9466(param_1);
    return;
}

pub unsafe fn handle_error_1008_04f8(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    handle_error_1008_0036(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn check_error_1008_087e(param_1: u16, param_2: *mut u16) {
    let bool_1: u8;
    let mut u_var1: u16;
    let mut local_SS__1: u16;
    let mut local_112: u16;
    let mut local_110: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    process_struct_1000_179c(10, param_2);
    u_var1 = param_2 | param_1;
    local_6 = param_1;
    local_4 = param_2;
    if (u_var1 != 0) {
        bool_1 = pass1_1030_8128(CONCAT22(param_2, param_1));
        param_1 = bool_1;
    }
    if (ctx._g_bool_1050_5748 == 0x0) {
        fn_1008_6048(
            ctx.s_New_failed_in_Op__Op__Simulator_1050_0130,
            u_var1,
            SUB21(param_1, 0),
        );
        call_fn_ptr_1000_24cd(1);
    }
    pass1_1028_e2e0(ctx._PTR_LOOP_1050_65e2);
    pass1_1028_e2e0(ctx._PTR_LOOP_1050_65e2);
    pass1_1030_532e(CONCAT22(local_SS__1, &local_112), 0xff000000);
    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(local_SS__1, &local_112));
    pass1_1030_838e(ctx._g_bool_1050_5748);
    local_112 = ctx.s_1_1050_389a;
    local_110 = &ctx.PTR_LOOP_1050_1008;
    pass1_1030_8334();
    return;
}

pub unsafe fn error_check_1008_3a7a(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1008_397a(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn error_check_1008_5fa2(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    modify_u16_list_1008_5c34(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn error_check_1008_8e74(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1008_8aa2(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn handle_error_1008_9466(param_1: *mut u16) {
    unsafe { *param_1 = 0x52a };
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    error_check_1000_17ce(ctx._PTR_LOOP_1050_0392);
    ctx._PTR_LOOP_1050_0392 = 0x0;
    return;
}

pub unsafe fn error_check_1008_ad64(param_1: u32, param_2: u8) {
    pass1_1008_a086(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn set_error_mode_1010_8b14(param_1: u32, param_2: *mut libc::c_char) -> *mut libc::c_char {
    let mut mode: u16;
    let mut u_var1: i32;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut unaff_ss: u16;
    let mut local_3c: u16;
    let mut local_3a: [u8; 44];
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    u_var3 = (param_1 >> 0x10);
    pass1_1008_5784(CONCAT22(unaff_ss, local_a), (param_1 + 0xe84));
    mode = SetErrorMode16(1);
    while {
        _local_e = pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
        if (_local_e == 0) {
            SetErrorMode16(mode);
            return param_2;
        }
        u_var1 = param_1 + 0xa82;
        copy_string_1000_3d3e((param_1 & 0xffff0000 | u_var1), *(_local_e + 4));
        process_string_1000_3cea((param_1 & 0xffff0000 | u_var1), param_2);
        i_var2 = dos3_call_1000_51aa(u_var1, u_var3, 1, local_3a);
        i_var2 != 0
    } {}
    SetErrorMode16(mode);
    return (param_1 & 0xffff0000 | u_var1);
}

pub fn set_error_mode_1010_85be(
    param_1: u32,
    in_struct_1: *mut Struct13,
    in_struct_1_hi: *mut Struct13,
) {
    let mut u_var1: u32;
    let mut unaff_ss: u16;
    let mut local_30e: u16;
    let mut local_30c: u16;
    let mut string_1: [u8; 256];
    let mut string_2: [u8; 256];
    let mut string_3: [u8; 260];
    let mut local_6: u16;
    let mut local_4: u16;

    if (in_struct_1 == &dos_alloc_addr_1050_0002) {
        u_var1 = (in_struct_1_hi * 4 + 0x2e34);
        process_string_1000_4d58(u_var1 & 0xffff0000 | (u_var1 + 3), 0, 0);
        copy_string_1000_3d3e(CONCAT22(unaff_ss, string_3), s_male_1050_14c6);
        process_string_1000_3cea(CONCAT22(unaff_ss, string_3), CONCAT22(unaff_ss, string_2));
        process_string_1000_3cea(CONCAT22(unaff_ss, string_3), CONCAT22(unaff_ss, string_1));
        set_error_mode_1010_8b14(param_1, string_3, unaff_ss);
        return;
    }
    set_error_mode_1010_8b14(param_1, (in_struct_1_hi * 4 + 0x2e34));
    return;
}

pub unsafe fn error_check_1040_a582(param_1: *mut u32) {
    let param_1_val = unsafe { *param_1 };
    error_check_1000_17ce(param_1_val);
    return;
}

pub unsafe fn cleanup_1040_a4c2(param_1: *mut Struct348, param_2: u8) -> *mut Struct348 {
    win_cleanup_fn_1040_a294(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn error_check_1040_8db6(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    error_check_1040_869a(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn error_check_1040_869a(param_1: *mut Struct363) {
    let local_bx_4: *mut Struct363;
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    param_1 = 0x8ddc;
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    error_check_1000_17ce(local_bx_4.field_0x90);
    error_check_1000_17ce(local_bx_4.field_0x94);
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub unsafe fn pass1_1030_e4be(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1030_e282(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1030_4538(param_1: *mut *mut Struct44) {
    let mut u_var1: u16;

    let param_1_val = unsafe { *param_1 };

    error_check_1000_17ce(param_1_val);
    u_var1 = (param_1 >> 0x10);
    error_check_1000_17ce((param_1 + 0x12));
    error_check_1000_17ce((param_1 + 0x15c));
    return;
}
