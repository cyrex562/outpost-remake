use crate::{mixed_fn_1010_830a, struct_ops1};
use crate::app_context::AppContext;
use crate::big_funcs::call_big_fn_1010_c7e2;
use crate::draw::draw1::load_cursor_1020_7f7a;
use crate::draw::draw2::process_struct_1020_1738;
use crate::err_funcs::error_check_1000_17ce;
use crate::func_ptr_funcs::call_fn_ptr_1000_5586;
use crate::list_funcs::zero_list_1008_6c90;
use crate::mem_funcs::Address;
use crate::other_funcs::zero_list_1008_3e38;
use crate::pass::pass14_funcs::{pass1_1008_3e94, pass1_1008_4d84, pass1_1008_57c4};
use crate::pass::pass19_funcs::pass1_1040_5d12;
use crate::pass::pass20_funcs::{pass1_1010_c864, pass1_1010_cc56, pass1_1010_cf36, pass1_1010_d24a, pass1_1010_d448, pass1_1010_d5ae, pass1_1010_d710};
use crate::pass::pass8_funcs::{pass1_1010_038e, pass1_1010_041a, pass1_1010_0538, pass1_1010_65d0};
use crate::pass::pass_funcs::pass1_1000_4906;
use crate::prog_structs::prog_structs_1::{Struct104, Struct552};
use crate::prog_structs::prog_structs_11::{Struct355, Struct475};
use crate::prog_structs::prog_structs_12::Struct1049;
use crate::prog_structs::prog_structs_13::Struct880;
use crate::prog_structs::prog_structs_14::Struct893;
use crate::prog_structs::prog_structs_15::Struct1169;
use crate::prog_structs::prog_structs_16::Struct1023;
use crate::prog_structs::prog_structs_17::{Struct1026, Struct1055, Struct1175};
use crate::prog_structs::prog_structs_18::Struct1042;
use crate::prog_structs::prog_structs_2::{Struct1054, Struct199, Struct660};
use crate::prog_structs::prog_structs_20::{Struct1073, Struct508};
use crate::prog_structs::prog_structs_23::{Struct1022, Struct1037};
use crate::prog_structs::prog_structs_24::{Struct1144, Struct1172, Struct2111, Struct354};
use crate::prog_structs::prog_structs_25::{Struct65, Struct882};
use crate::prog_structs::prog_structs_27::Struct1029;
use crate::prog_structs::prog_structs_28::{Struct1034, Struct1040, Struct1043, Struct1044, Struct1046, Struct1047, Struct1048, Struct1056, Struct1057, Struct1071, Struct1074, Struct346, Struct357, Struct464};
use crate::prog_structs::prog_structs_29::{Struct1025, Struct1027, Struct1030, Struct1031, Struct1032, Struct1033, Struct1038, Struct1039};
use crate::prog_structs::prog_structs_31::{Struct352, Struct474};
use crate::prog_structs::prog_structs_5::Struct881;
use crate::prog_structs::prog_structs_6::Struct1041;
use crate::prog_structs::prog_structs_7::{Struct372, Struct376, Struct44, Struct629};
use crate::prog_structs::prog_structs_8::{Struct649, Struct68};
use crate::prog_structs::prog_structs_9::{Struct1019, Struct1072, Struct636};
use crate::string_ops1::copy_string_1000_3d3e;
use crate::sys1::get_sys_metrics_1018_4b1e;
use crate::sys2::process_struct_1040_8478;
use crate::ui_funcs::ui2::pass1_1038_af40;
use crate::util::{CARRY1, CONCAT11, CONCAT22, SBORROW1, SBORROW2, SUB42, ZEXT24};
use crate::winapi_funcs::GetSystemMetrics16;

pub fn process_struct_1010_95aa(param_1: *mut Struct464, param_2: u32) {
    let mut u_var1: u16;

    u_var1 = param_2;
    struct_ops1::process_struct_1010_1d48(CONCAT22(u_var1, param_1), (param_2 >> 0x10));
    param_1.b = 0;
    param_1.c = 0;
    param_1.d = 0;
    param_1.e = 0;
    param_1.f = 0;
    param_1.g = 0;
    param_1.h = 10;
    param_1.j = 0;
    CONCAT22(u_var1, param_1) = 0xa1c8;
    param_1.a = 0x1010;
    return;
}

pub fn process_struct_1010_9fee(param_1: u32, param_2: u16, param_3: u16) {
    let pp_var1: fn();
    let in_ax: *mut Struct199;
    let in_dx: *mut Struct199;
    let struct_a: *mut Struct199;
    let local_bx_4: *mut Struct474;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    u_var3 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    struct_a = in_dx;
    if (&local_bx_4.field_0x12 == 0) {
        struct_ops1::process_struct_1000_179c(0xc, in_dx);
        struct_a = (in_dx | in_ax);
        if (struct_a == 0x0) {
            &local_bx_4.field_0x12 = 0;
        } else {
            in_ax = struct_ops1::process_struct_1008_574a(CONCAT22(in_dx, in_ax));
            local_bx_4.field_0x12 = in_ax;
            &local_bx_4.field_0x14 = struct_a;
        }
    }
    struct_ops1::process_struct_1000_179c(8, struct_a);
    _local_a = CONCAT22(struct_a, in_ax);
    if ((struct_a | in_ax) == 0) {
        local_6 = 0;
    } else {
        *_local_a = ctx.s_1_1050_389a;
        in_ax.field_0x2 = &ctx.PTR_LOOP_1050_1008;
        *_local_a = 0xa1c4;
        in_ax.field_0x2 = 0x1010;
        local_6 = _local_a;
    }
    u_var4 = (local_6 >> 0x10);
    i_var2 = local_6;
    (i_var2 + 4) = param_3;
    (i_var2 + 6) = param_2;
    pp_var1 = (&local_bx_4.field_0x12 + 4);
    (**pp_var1)(0x1000, &local_bx_4.field_0x12, i_var2, u_var4);
    return;
}

pub fn process_struct_1010_a1d8(
    in_struct_a_1: *mut Struct475,
    in_struct_a_2: *mut Struct475,
    param_3: u16,
) {
    let pp_var1: fn();
    let string_ptr_base: String;
    let struct_a: *mut Struct2111;
    let mut local_56: u16;
    let mut local_54: u16;
    let mut local_52: u32;
    let mut local_4e: u16;
    let mut local_4c: u16;
    let mut local_4a: u16;
    let mut local_48: u32;
    let mut local_44: u16;
    let mut local_42: u16;
    let mut local_40: u16;
    let mut local_3e: u32;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u32;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u32;
    let mut local_26: u16;
    let mut local_24: u32;
    let mut local_20: u16;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_18: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_27fdf014d48: *mut Struct1175;

    struct_ops1::process_struct_1010_1d48(CONCAT22(in_struct_a_2, in_struct_a_1), param_3);
    &in_struct_a_1.field_0xa = ctx.s_1_1050_389a;
    (&in_struct_a_1.field_0xa + 2) = &ctx.PTR_LOOP_1050_1008;
    &in_struct_a_1.field_0xa = (ctx.s_18_2_1050_3aa5 + 3);
    (&in_struct_a_1.field_0xa + 2) = &ctx.PTR_LOOP_1050_1008;
    &in_struct_a_1.field_0x138 = 0;
    CONCAT22(in_struct_a_2, in_struct_a_1) = 0xe9cc;
    in_struct_a_1.field_0x2 = 0x1010;
    &in_struct_a_1.field_0xa = 0xe9dc;
    (&in_struct_a_1.field_0xa + 2) = 0x1010;
    struct_a =
        process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(string_ptr_base, 0x2f));
    &in_struct_a_1.field_0x138 = struct_a;
    (in_struct_a_1 + 1).field_0x0 = (struct_a >> 0x10);
    pp_var1 = (&in_struct_a_1.field_0x138 + 4);
    (**pp_var1)();
    pass1_1000_4906(CONCAT22(in_struct_a_2, &in_struct_a_1.field_0xa4), 0, 0x94);
    pass1_1000_4906(CONCAT22(in_struct_a_2, &in_struct_a_1.field_0xe), 0, 0x96);
    local_4 = 0;
    while {
        temp_27fdf014d48 = (&in_struct_a_1.field_0x0 + local_4 * 3);
        temp_27fdf014d48.field_0xe = call_big_fn_1010_c7e2;
        temp_27fdf014d48.field_0x12 = 0;
        local_4 = local_4 + 1;
        local_4 < 0x19
    } {}
    *(&in_struct_a_1.field_0x48 + 2) = pass1_1010_c864;
    &in_struct_a_1.field_0x4e = 0;
    *(&in_struct_a_1.field_0x4e + 2) = pass1_1010_cc56;
    &in_struct_a_1.field_0x54 = 0;
    *(&in_struct_a_1.field_0x54 + 2) = pass1_1010_cf36;
    &in_struct_a_1.field_0x5a = 0;
    *(&in_struct_a_1.field_0x2a + 2) = pass1_1010_d24a;
    &in_struct_a_1.field_0x30 = 0;
    *(&in_struct_a_1.field_0x6c + 2) = pass1_1010_d448;
    &in_struct_a_1.field_0x72 = 0;
    *(&in_struct_a_1.field_0x72 + 2) = pass1_1010_d5ae;
    &in_struct_a_1.field_0x78 = 0;
    *(&in_struct_a_1.field_0x96 + 2) = pass1_1010_d710;
    &in_struct_a_1.field_0x9c = 0;
    return;
}

pub fn process_struct_1018_0570(param_1: *mut u16, param_2: u16) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let pu_var3: *mut u16;
    let mut i_var4: i32;
    let struct_a: *mut Struct199;
    let pa_var5: *mut Struct199;
    let mut u_var6: u16;
    let local_bx_18: *mut Struct508;
    let pp_var7: *mut Struct2111;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut in_stack_0000fff0: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var9 = (param_1 >> 0x10);
    get_sys_metrics_1018_4b1e(param_1, u_var9, 0, param_2);
    local_bx_18 = param_1;
    local_bx_18.field_0x20 = ctx.s_1_1050_389a;
    local_bx_18.field_0x22 = &ctx.PTR_LOOP_1050_1008;
    local_bx_18.field_0x20 = (ctx.s_18_2_1050_3aa5 + 3);
    local_bx_18.field_0x22 = &ctx.PTR_LOOP_1050_1008;
    local_bx_18.field_0x24 = 0;
    &local_bx_18.field_0x2c = 0;
    zero_list_1008_3e38((param_1 & 0xffff0000 | &local_bx_18.field_0x30));
    zero_list_1008_3e38((param_1 & 0xffff0000 | &local_bx_18.field_0x36));
    local_bx_18.field_0x3c = 0;
    zero_list_1008_6c90(&local_bx_18.field_0x40, u_var9);
    u_var6 = 0;
    local_bx_18.field_0x4c = 0;
    local_bx_18.field_0x5a = 0;
    local_bx_18.field_0x5e = 0;
    local_bx_18.field_0x60 = 0;
    local_bx_18.field_0x64 = 0xff00;
    local_bx_18.field_0x66 = 0;
    local_bx_18.field_0x68 = 0x10000fb;
    local_bx_18.field_0x6c = 0x10000f9;
    local_bx_18.field_0x70 = 0x10000ff;
    local_bx_18.field_0x74 = 0x10000fe;
    local_bx_18.field_0x78 = 0x10000fc;
    local_bx_18.field_0x7c = 0;
    local_bx_18.field_0x80 = 0;
    local_bx_18.field_0x84 = 1;
    local_bx_18.field_0x86 = 0;
    local_bx_18.field_0x88 = 0;
    local_bx_18.field_0x8c = 0;
    local_bx_18.field_0x8e = 0;
    local_bx_18.field_0x92 = 0;
    local_bx_18.field_0x94 = 0;
    local_bx_18.field_0x98 = 0;
    local_bx_18.field_0x9a = 0;
    &local_bx_18.field_0xa2 = 0;
    local_bx_18.field_0xa6 = 0xffff;
    local_bx_18.field_0xa8 = 0;
    unsafe {
        *param_1 = (s_582_bmp_1050_1871 + 3);
    }
    local_bx_18.field_0x2 = 0x1018;
    local_bx_18.field_0x20 = (s_589_bmp_1050_18a9 + 7);
    local_bx_18.field_0x22 = 0x1018;
    if ((PTR_LOOP_1050_3960 == 0x0) && (_PTR_LOOP_1050_3962 == 0)) {
        pa_var5 = struct_a;
        struct_ops1::process_struct_1000_179c(8, struct_a);
        _PTR_LOOP_1050_3962 = CONCAT22(pa_var5, u_var6);
        pass1_1000_4906(CONCAT22(pa_var5, u_var6), 0, 8);
    }
    PTR_LOOP_1050_3960 = PTR_LOOP_1050_3960 + 1;
    pp_var7 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22(in_stack_0000fff0, 0x2f),
    );
    local_bx_18.field_0x2c = pp_var7;
    &local_bx_18.field_0x2e = (pp_var7 >> 0x10);
    if (param_1 == 0x0) {
        pu_var3 = 0x0;
        u_var6 = 0;
    } else {
        pu_var3 = &local_bx_18.field_0x20;
        u_var6 = u_var9;
    }
    u_var1 = &local_bx_18.field_0x2c;
    u_var8 = u_var1;
    ppc_var2 = (&local_bx_18.field_0x2c + 4);
    ppc_var2(0x10, u_var8, (u_var1 >> 0x10), 0, pu_var3, u_var6);
    pp_var7 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(u_var8, 2));
    if ((pp_var7 + 0x80) != 0) {
        local_bx_18.field_0x84 = 2;
    }
    pp_var7 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(u_var8, 9));
    i_var4 = pp_var7;
    local_bx_18.field_0x9e = i_var4;
    local_bx_18.field_0xa0 = (pp_var7 >> 0x10);
    pass1_1010_65d0(pp_var7 & 0xffff0000 | local_bx_18.field_0x9e, 0x88);
    if (i_var4 != 0) {
        local_bx_18.field_0xa8 = 1;
    }
    pp_var7 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(u_var8, 0x3b));
    local_bx_18.field_0xa2 = pp_var7;
    local_bx_18.field_0xa4 = (pp_var7 >> 0x10);
    return;
}

pub fn set_struct_1018_36e6(in_struct_1: *mut Struct552, param_2: u16, param_3: u16, param_4: u16) {
    let struct_a_2: *mut Struct552;
    let struct_a_1: *mut Struct552;

    struct_a_1 = (in_struct_1 >> 0x10);
    struct_a_2 = in_struct_1;
    struct_a_2.field_0x12e = param_4;
    struct_a_2.field_0x130 = param_3;
    struct_a_2.field_0x132 = param_2;
    struct_a_2.field_0x134 = 0;
    return;
}

pub unsafe fn process_struct_1018_e2a0(in_Struct376: *mut Struct376) {
    let local_Struct376_lo: *mut Struct376;
    let local_Struct376_hi: *mut Struct376;

    local_Struct376_hi = (in_Struct376 >> 0x10);
    local_Struct376_lo = in_Struct376;
    in_Struct376.ptr_a_lo = 0xe44e;
    local_Struct376_lo.ptr_a_hi = 0x1018;
    local_Struct376_lo.offset_xe2 = 0xe4ea;
    local_Struct376_lo.segment_xe4 = 0x1018;
    process_struct_1020_808e(in_Struct376);
    return;
}

pub unsafe fn process_struct_1018_e428(
    in_Struct376_ptr_1: *mut Struct376,
    param_2: u8,
) -> *mut Struct376 {
    process_struct_1018_e2a0(in_Struct376_ptr_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(in_Struct376_ptr_1);
    }
    return in_Struct376_ptr_1;
}

pub fn process_struct_1018_e5dc(in_struct_1: *mut Struct65, param_2: u16, param_3: u32) {
    let mut u_var1: u16;
    let mut i_var2: i32;
    let mut unaff_bp: u16;
    let local_struct_1: *mut Struct65;
    let ppVar3: *mut Struct2111;

    load_cursor_1020_7f7a(in_struct_1, CONCAT22(param_2, 9), param_3);
    local_struct_1 = (in_struct_1 >> 0x10);
    i_var2 = in_struct_1;
    (i_var2 + 0xee) = 0;
    (i_var2 + 0xf2) = 0;
    in_struct_1.ptr_a_lo = 0xe790;
    (i_var2 + 2) = 0x1018;
    (i_var2 + 0xe2) = 0xe82c;
    (i_var2 + 0xe4) = 0x1018;
    ppVar3 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_bp, 10));
    u_var1 = (ppVar3 >> 0x10);
    (i_var2 + 0xf2) = ppVar3;
    (i_var2 + 0xf4) = u_var1;
    (i_var2 + 0xe6) = (i_var2 + 0xf2);
    (i_var2 + 0xe8) = u_var1;
    return;
}

pub unsafe fn process_struct_1018_e64c(param_1: &mut Struct44) {
    let local_bx_3: *mut Struct376;
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    local_bx_3 = param_1;
    param_1.ptr_a_lo = 0xe790;
    local_bx_3.ptr_a_hi = 0x1018;
    local_bx_3.offset_xe2 = 0xe82c;
    local_bx_3.segment_xe4 = 0x1018;
    process_struct_1020_808e(param_1);
    return;
}

pub unsafe fn process_struct_1018_e76a(in_struct_1: *mut Struct376, param_2: u8) -> *mut Struct376 {
    process_struct_1018_e64c(in_struct_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(in_struct_1);
    }
    return in_struct_1;
}

pub fn process_struct_1018_e91e(in_struct_a: *mut Struct65, param_2: u16, param_3: u32) {
    let mut u_var1: u32;
    let paVar2: *mut Struct65;
    let pu_var3: *mut u16;
    let mut u_var4: u16;
    let pa_var5: *mut Struct65;
    let ppVar6: *mut Struct2111;
    let struct_a_1: *mut Struct65;
    let struct_a_2: *mut Struct65;
    let mut uStack14: u16;
    let char_ptr_1: String;
    // fn_ptr_1: *mut Vec<u8>;

    struct_a_1 = in_struct_a;
    struct_a_2 = (in_struct_a >> 0x10);
    paVar2 = struct_a_2;
    load_cursor_1020_7f7a(in_struct_a, CONCAT22(param_2, 3), param_3);
    &struct_a_1.u16_xee = 0;
    &struct_a_1.u16_xf2 = 0;
    &struct_a_1.field_0xf6 = 0;
    in_struct_a.ptr_a_lo = 0xebd0;
    struct_a_1.ptr_a_hi = 0x1018;
    struct_a_1.ptr_b_lo = 0xec6c;
    struct_a_1.ptr_b_hi = 0x1018;
    ppVar6 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(char_ptr_1, 0x28));
    u_var4 = (ppVar6 >> 0x10);
    struct_a_1.u16_xf2 = ppVar6;
    struct_a_1.u16_xf4 = u_var4;
    struct_a_1.u16_xe6 = struct_a_1.u16_xf2;
    struct_a_1.u16_xe8 = u_var4;
    ppVar6 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(char_ptr_1, 0x32));
    struct_a_1.field_0xf6 = ppVar6;
    struct_a_1.field_0xf8 = (ppVar6 >> 0x10);
    if (in_struct_a == 0x0) {
        pu_var3 = 0x0;
        pa_var5 = 0x0;
    } else {
        pu_var3 = &struct_a_1.ptr_b_lo;
        pa_var5 = struct_a_2;
    }
    u_var1 = &struct_a_1.field_0xf6;
    struct_a_2 = u_var1;
    uStack14 = (u_var1 >> 0x10);
    fn_ptr_1 = (&struct_a_1.field_0xf6 + 4);
    (**fn_ptr_1)(0x1010, struct_a_2, uStack14, 0xb, pu_var3, pa_var5);
    return;
}

pub fn process_struct_1020_04f6(in_struct_1: *mut Struct629, param_2: u16) {
    let pp_var1: fn();
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let local_struct_1: *mut Struct629;
    let mut local_struct_1_hi: u16;
    let ppVar5: *mut Struct2111;
    let mut u_var6: u16;
    let local_1e: String;
    let pcStack32: String;
    let mut local_6: u16;
    let mut local_4: u16;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    in_struct_1.u16_x00 = ctx.s_1_1050_389a;
    local_struct_1.u16_x02 = &ctx.PTR_LOOP_1050_1008;
    in_struct_1.u16_x00 = (ctx.s_18_2_1050_3aa5 + 3);
    local_struct_1.u16_x02 = &ctx.PTR_LOOP_1050_1008;
    local_struct_1.u16_x04 = param_2;
    in_struct_1.u16_x00 = ctx.s_0_020_1050_3ab0;
    local_struct_1.u16_x02 = &ctx.PTR_LOOP_1050_1008;
    &local_struct_1.u16_x06 = 0;
    local_struct_1.u16_x0A = 0;
    local_struct_1.u16_x0C = 0;
    local_struct_1.u16_x0E = 0;
    local_struct_1.u16_x10 = 0;
    in_struct_1.u16_x00 = 0x75a;
    local_struct_1.u16_x02 = 0x1020;
    pcStack32 = CONCAT22(local_1e, 1);
    ppVar5 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, pcStack32);
    u_var3 = (ppVar5 >> 0x10);
    local_struct_1.u16_x06 = ppVar5;
    local_struct_1.u16_x08 = u_var3;
    u_var6 = local_struct_1.u16_x06;
    pp_var1 = (&local_struct_1.u16_x06 + 4);
    (**pp_var1)(0x1010, u_var6, u_var3, 0, in_struct_1, local_1e);
    ppVar5 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(u_var6, 0x48));
    u_var4 = (ppVar5 >> 0x10);
    i_var2 = ppVar5;
    local_struct_1.u16_x0A = (i_var2 + 10);
    local_struct_1.u16_x0C = (i_var2 + 0xc);
    pass1_1008_3e94(
        (i_var2 + 0xe),
        (in_struct_1 & 0xffff0000 | ZEXT24(&local_struct_1.u16_x10)),
        (in_struct_1 & 0xffff0000 | ZEXT24(&local_struct_1.u16_x0E)),
    );
    return;
}

pub fn process_struct_1020_0baa(in_struct_1: *mut Struct636, param_2: u16) {
    let mut local_DX_123: u16;
    let local_struct_1: *mut Struct636;
    let local_struct_1_hi: *mut Struct636;
    let ppVar1: *mut Struct2111;
    let p_uvar2: *mut u16;
    let pu_var3: *mut u16;
    let paVar4: *mut Struct636;
    let local_string_1: String;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    in_struct_1.u16_0x0 = ctx.s_1_1050_389a;
    local_struct_1.u16_0x02 = &ctx.PTR_LOOP_1050_1008;
    in_struct_1.u16_0x0 = (ctx.s_18_2_1050_3aa5 + 3);
    local_struct_1.u16_0x02 = &ctx.PTR_LOOP_1050_1008;
    local_struct_1.u16_0x04 = param_2;
    in_struct_1.u16_0x0 = ctx.s_0_020_1050_3ab0;
    local_struct_1.u16_0x02 = &ctx.PTR_LOOP_1050_1008;
    &local_struct_1.u16_0x06 = 0;
    local_struct_1.u16_0x0a = 0;
    local_struct_1.u16_0x0c = 0;
    in_struct_1.u16_0x0 = 0xdbc;
    local_struct_1.u16_0x02 = 0x1020;
    ppVar1 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_string_1, 7));
    local_struct_1.u16_0x06 = ppVar1;
    local_struct_1.u16_0x08 = (ppVar1 >> 0x10);
    pu_var3 = &local_struct_1.u16_0x0a;
    pu_var2 = &local_struct_1.u16_0x0c;
    paVar4 = local_struct_1_hi;
    ppVar1 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(pu_var2, 0x48));
    pass1_1008_3e94(
        (ppVar1 + 0xe),
        CONCAT22(local_struct_1_hi, pu_var2),
        CONCAT22(paVar4, pu_var3),
    );
    return;
}

pub fn process_struct_1040_a640(in_struct_1: *mut Struct352, param_2: u32, param_3: u16) {
    let local_struct_1: *mut Struct352;
    let mut local_es_22: u16;

    struct_ops1::process_struct_1040_b082(in_struct_1, CONCAT22(param_3, 0x1f1));
    local_es_22 = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    local_struct_1.field_0x94 = param_2;
    local_struct_1.field_0x98 = 0;
    local_struct_1.field_0xea = 0;
    in_struct_1 = 0xac08;
    local_struct_1.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    return;
}

pub fn process_struct_1040_a598(param_1: *mut Struct354) {
    let local_bx_3: *mut Struct354;
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    local_bx_3 = param_1;
    param_1 = 0;
    local_bx_3.field_0x2 = 0;
    local_bx_3.field_0x6 = 0;
    local_bx_3.field_0xa = 0;
    local_bx_3.field_0xc = 0;
    local_bx_3.field_0x10 = 0;
    local_bx_3.field_0x12 = 0;
    local_bx_3.field_0x14 = 0;
    local_bx_3.field_0x16 = 0;
    return;
}

pub fn process_struct_1040_9824(param_1: *mut u32) {
    let local_bx_3: *mut Struct355;
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    local_bx_3 = param_1;
    unsafe {
        *param_1 = 0;
    }
    local_bx_3.field_0x4 = 0;
    local_bx_3.field_0x56 = 0;
    local_bx_3.field_0x5a = 0;
    local_bx_3.field_0x5c = 0;
    *&local_bx_3.field_0x6 = 0;
    return;
}

pub fn process_struct_1040_9618(param_1: *mut Struct357) {
    let mut u_var1: u16;
    let local_bx_4: *mut Struct357;
    let mut u_var2: u16;
    let mut u_var3: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var2 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    u_var3 = struct_ops1::process_struct_1008_4772(local_bx_4.field_0x8);
    u_var1 = (u_var3 >> 0x10);
    local_bx_4.field_0x2a = (u_var3 + 4);
    local_bx_4.field_0x2c = (u_var3 + 8);
    return;
}

pub fn process_struct_1040_807e(param_1: u32, param_2: u16) {
    let mut u_var1: i32;
    let ppc_var2: fn();
    let in_ax: *mut u32;
    let pu_var3: *mut u32;
    let pu_var4: *mut u32;

    let ctx.dx_reg: *mut Struct199;
    let struct_a: *mut Struct199;
    let ctx.dx_reg: *mut Struct199;

    let mut u_var5: u16;
    let mut i32_var6: i32;
    let mut u_var7: u16;
    let mut local_10: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2 == 1) {
        pass1_1040_805a();
        return;
    }
    mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, param_2);
    local_6 = CONCAT22(ctx.dx_reg, in_ax);
    if ((ctx.dx_reg | in_ax) != 0) {
        ppc_var2 = (*local_6 + 0x14);
        pu_var3 = in_ax;
        ppc_var2(0x1010, in_ax, ctx.dx_reg);
        u_var7 = (param_1 >> 0x10);
        i32_var6 = param_1;
        struct_a = ctx.dx_reg;
        pu_var4 = pu_var3;
        if ((i32_var6 + 0x70) != 0) {
            pu_var4 = (i32_var6 + 0x70);
            u_var1 = (i32_var6 + 0x72);
            struct_a = (u_var1 | pu_var4);
            if (struct_a != 0x0) {
                unsafe {
                    ppc_var2 = *pu_var4;
                }
                ppc_var2(0x1010, pu_var4, u_var1, 1);
                struct_a = ctx.dx_reg;
            }
        }
        struct_ops1::process_struct_1000_179c(0x14, struct_a);
        if ((struct_a | pu_var4) == 0) {
            pu_var4 = 0x0;
            u_var5 = 0;
        } else {
            struct_ops1::process_struct_1008_4c58(CONCAT22(struct_a, pu_var4));
            u_var5 = ctx.dx_reg;
        }
        (i32_var6 + 0x70) = pu_var4;
        (i32_var6 + 0x72) = u_var5;
        pass1_1008_4d84((i32_var6 + 0x70), pu_var3);
        if (local_6 != 0x0) {
            ppc_var2 = *local_6;
            ppc_var2(&ctx.PTR_LOOP_1050_1008, in_ax, ctx.dx_reg, 1);
        }
        return;
    }
    return;
}

pub fn process_struct_1040_7728(
    in_struct_1: *mut Struct68,
    param_2: Vec<u8>,
    param_3: u32,
    param_4: Vec<u8>,
    param_5: Vec<u8>,
) -> u8 {
    let iVar1: u16;
    let local_struct_1: *mut Struct68;
    let local_struct_1_hi: *mut Struct68;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    in_struct_1.field_0x0 = ctx.s_1_1050_389a;
    local_struct_1.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    in_struct_1.field_0x0 = (ctx.s_18_2_1050_3aa5 + 3);
    local_struct_1.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    local_struct_1.field_0x4 = 0;
    local_struct_1.field_0x6 = 0;
    local_struct_1.field_0x8 = param_5;
    local_struct_1.field_0xa = param_4;
    local_struct_1.field_0xc = 0;
    local_struct_1.field_0x60 = 0;
    local_struct_1.field_0x62 = 0;
    local_struct_1.field_0x64 = 0;
    local_struct_1.field_0x66 = 0;
    local_struct_1.field_0x68 = 0;
    local_struct_1.field_0x6a = param_3;
    local_struct_1.field_0x6e = param_2;
    local_struct_1.field_0x70 = 0;
    local_struct_1.field_0x74 = 0;
    local_struct_1.field_0x76 = 0;
    local_struct_1.field_0x78 = 0;
    local_struct_1.field_0x8a = 0;
    local_struct_1.field_0x8c = 0;
    in_struct_1.field_0x0 = 0x840c;
    local_struct_1.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    copy_string_1000_3d3e(
        (in_struct_1 & 0xffff0000 | &local_struct_1.field_0x10),
        0x10505db0,
    );
    pass1_1000_4906(
        (in_struct_1 & 0xffff0000 | ZEXT24(&local_struct_1.struct_ptr_0x7a)),
        0,
        8,
    );
    pass1_1000_4906(
        (in_struct_1 & 0xffff0000 | ZEXT24(&local_struct_1.struct_ptr_0x82)),
        0,
        8,
    );
    iVar1 = GetSystemMetrics16(4);
    local_struct_1.field_0x62 = iVar1;
    iVar1 = GetSystemMetrics16(5);
    local_struct_1.field_0x64 = iVar1;
    iVar1 = GetSystemMetrics16(6);
    local_struct_1.field_0x66 = iVar1;
    return in_struct_1;
}

pub fn pass1_1040_6fb6(param_1: *mut Struct346, param_2: Vec<u8>) {
    let mut iVar1: i32;
    let mut u_var2: u16;

    struct_ops1::process_struct_1040_b0bc(param_1, 0, CONCAT22(param_2, 0xfd9));
    u_var2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x94) = 0;
    (iVar1 + 0x98) = 0;
    param_1 = 0x76a4;
    (iVar1 + 2) = &ctx.PTR_LOOP_1050_1040;
    return;
}

pub fn pass1_1040_5626(param_1: *mut u16, param_2: Vec<u8>, param_3: u16) {
    let mut u_var1: u32;
    let mut u_var2: i32;
    let struct_a: *mut Struct199;
    let paVar3: *mut Struct199;
    let struct_a_00: *mut Struct199;

    let mut i_var4: i32;
    let mut i_var5: i32;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut u_var8: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    struct_ops1::process_struct_1040_b082(param_1, CONCAT22(param_3, 0xfa3));
    u_var6 = (param_1 >> 0x10);
    i_var4 = param_1;
    u_var2 = 0;
    (i_var4 + 0x94) = 0;
    (i_var4 + 0x96) = 0;
    (i_var4 + 0x98) = 0;
    (i_var4 + 0x9c) = 0;
    unsafe {
        *param_1 = 0x6386;
    }
    (i_var4 + 2) = &ctx.PTR_LOOP_1050_1040;
    paVar3 = struct_a;
    struct_ops1::process_struct_1000_179c(0x18, struct_a);
    struct_a_00 = (paVar3 | u_var2);
    if (struct_a_00 == 0x0) {
        (i_var4 + 0x90) = 0;
    } else {
        process_struct_1040_a598(CONCAT22(paVar3, u_var2));
        (i_var4 + 0x90) = u_var2;
        (i_var4 + 0x92) = ctx.dx_reg;
        struct_a_00 = ctx.dx_reg;
    }
    (i_var4 + 0x90) = 6;
    i_var5 = (i_var4 + 0x90);
    u_var2 = i_var5 * 10 + 2;
    struct_ops1::process_struct_1000_179c(u_var2, struct_a_00);
    _local_c = CONCAT22(struct_a_00, u_var2);
    if ((struct_a_00 | u_var2) == 0) {
        u_var1 = (i_var4 + 0x90);
        (u_var1 + 2) = 0;
    } else {
        *_local_c = i_var5;
        call_fn_ptr_1000_5586(
            0xa564,
            &ctx.PTR_LOOP_1050_1040,
            i_var5,
            10,
            u_var2 + 2,
            struct_a_00,
        );
        u_var1 = (i_var4 + 0x90);
        u_var7 = (u_var1 >> 0x10);
        i_var5 = u_var1;
        (i_var5 + 2) = u_var2 + 2;
        (i_var5 + 4) = struct_a_00;
    }
    u_var1 = (i_var4 + 0x90);
    *(u_var1 + 6) = param_2;
    u_var1 = (i_var4 + 0x90);
    (u_var1 + 10) = 4;
    u_var1 = (i_var4 + 0x90);
    (u_var1 + 0x12) = (i_var4 + 10);
    u_var8 = pass1_1040_5d12(param_1);
    u_var2 = (u_var8 >> 0x10);
    if ((u_var2 | u_var8) == 0) {
        (i_var4 + 0x9a) = 0;
    } else {
        (i_var4 + 0x9a) = (u_var8 + 0x20);
    }
    return;
}

pub fn pass1_1040_44d2(param_1: *mut u16, param_2: Vec<u8>, param_3: Vec<u8>) {
    let mut u_var1: u32;
    let mut in_ax: i32;
    let mut u_var2: i32;
    let struct_a: *mut Struct199;
    let paVar3: *mut Struct199;
    let struct_a_00: *mut Struct199;

    let mut i_var4: i32;
    let mut i_var5: i32;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    struct_ops1::process_struct_1040_b082(param_1, CONCAT22(param_3, 0xfa2));
    u_var6 = (param_1 >> 0x10);
    i_var4 = param_1;
    unsafe {
        *param_1 = 0x4824;
    }
    (i_var4 + 2) = &ctx.PTR_LOOP_1050_1040;
    paVar3 = struct_a;
    struct_ops1::process_struct_1000_179c(0x18, struct_a);
    struct_a_00 = (paVar3 | in_ax);
    if (struct_a_00 == 0x0) {
        (i_var4 + 0x90) = 0;
    } else {
        process_struct_1040_a598(CONCAT22(paVar3, in_ax));
        (i_var4 + 0x90) = in_ax;
        (i_var4 + 0x92) = ctx.dx_reg;
        struct_a_00 = ctx.dx_reg;
    }
    (i_var4 + 0x90) = 0x14;
    i_var5 = (i_var4 + 0x90);
    u_var2 = i_var5 * 10 + 2;
    struct_ops1::process_struct_1000_179c(u_var2, struct_a_00);
    _local_8 = CONCAT22(struct_a_00, u_var2);
    if ((struct_a_00 | u_var2) == 0) {
        u_var1 = (i_var4 + 0x90);
        (u_var1 + 2) = 0;
    } else {
        *_local_8 = i_var5;
        call_fn_ptr_1000_5586(
            0xa564,
            &ctx.PTR_LOOP_1050_1040,
            i_var5,
            10,
            u_var2 + 2,
            struct_a_00,
        );
        u_var1 = (i_var4 + 0x90);
        u_var7 = (u_var1 >> 0x10);
        i_var5 = u_var1;
        (i_var5 + 2) = u_var2 + 2;
        (i_var5 + 4) = struct_a_00;
    }
    u_var1 = (i_var4 + 0x90);
    *(u_var1 + 6) = param_2;
    u_var1 = (i_var4 + 0x90);
    (u_var1 + 10) = 1;
    u_var1 = (i_var4 + 0x90);
    (u_var1 + 0x12) = (i_var4 + 10);
    return;
}

pub fn pass1_1040_34a2(
    param_1: *mut Struct68,
    param_2: u32,
    param_3: Vec<u8>,
    param_4: Vec<u8>,
    param_3_00: Vec<u8>,
) {
    let mut iVar1: i32;
    let mut unaff_bp: u16;
    let mut u_var2: u16;
    let ppVar3: *mut Struct2111;

    process_struct_1040_7728(
        param_1,
        (&ctx.PTR_LOOP_1050_0000 + 1),
        param_2,
        (s_You_may_not_run_a_turn__The_game_1050_0172 + 0x20),
        param_3_00,
    );
    u_var2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    (iVar1 + 0x92) = 0;
    (iVar1 + 0x94) = 0;
    (iVar1 + 0x96) = 0;
    (iVar1 + 0x98) = 0;
    param_1.field_0x0 = (s_Null_Ptr_1050_38f3 + 7);
    (iVar1 + 2) = &ctx.PTR_LOOP_1050_1040;
    ppVar3 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_bp, 0x3c));
    (iVar1 + 0x8e) = ppVar3;
    (iVar1 + 0x90) = (ppVar3 >> 0x10);
    return;
}

pub fn pass1_1040_2ea2(
    param_1: *mut Struct68,
    param_2: u32,
    param_3: Vec<u8>,
    param_4: Vec<u8>,
    param_3_00: Vec<u8>,
) {
    let mut iVar1: i32;
    let mut unaff_bp: u16;
    let mut u_var2: u16;
    let ppVar3: *mut Struct2111;

    process_struct_1040_7728(
        param_1,
        (&ctx.PTR_LOOP_1050_0000 + 1),
        param_2,
        (s_You_may_not_run_a_turn__The_game_1050_0172 + 0xe),
        param_3_00,
    );
    u_var2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    (iVar1 + 0x90) = 0;
    (iVar1 + 0x92) = 0;
    (iVar1 + 0x94) = 0;
    (iVar1 + 0x96) = 0;
    param_1.field_0x0 = 0x3436;
    (iVar1 + 2) = &ctx.PTR_LOOP_1050_1040;
    ppVar3 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_bp, 0x3c));
    (iVar1 + 0x96) = ppVar3;
    (iVar1 + 0x98) = (ppVar3 >> 0x10);
    return;
}

pub fn pass1_1040_1f5a(param_1: *mut Struct68, param_2: Vec<u8>) {
    let piVar1: *mut i32;
    let mut u_var2: u16;

    let mut i_var3: i32;
    let mut unaff_si: u16;
    let mut u_var4: u16;
    let mut u_var5: u32;
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
    let mut local_6: u16;
    let mut local_4: u16;

    process_struct_1040_7728(param_1, (&ctx.PTR_LOOP_1050_0000 + 1), 0, 0xfcf, param_2);
    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    u_var2 = 0;
    (i_var3 + 0x8e) = 0;
    (i_var3 + 0xa2) = 0;
    (i_var3 + 0xa6) = 0;
    param_1.field_0x0 = (s_alarm_m_1050_2377 + 7);
    (i_var3 + 2) = &ctx.PTR_LOOP_1050_1040;
    mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, 0x1cc);
    (i_var3 + 0x8e) = u_var2;
    (i_var3 + 0x90) = ctx.dx_reg;
    u_var5 = struct_ops1::process_struct_1008_4772(CONCAT22(ctx.dx_reg, (i_var3 + 0x8e)));
    u_var2 = (u_var5 >> 0x10);
    process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x48));
    _local_16 = CONCAT22((u_var5 + 8) + 10, 10);
    _local_12 = CONCAT22(0x1d6, (u_var5 + 4) + -10);
    (i_var3 + 0x92) = _local_16;
    (i_var3 + 0x96) = _local_12;
    (i_var3 + 0x9a) = _local_16;
    (i_var3 + 0x9e) = _local_12;
    piVar1 = (i_var3 + 0x9c);
    unsafe {
        *piVar1 = *piVar1 + 0x14;
    }
    process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(i_var3 + 0xa6, 0x2b));
    pass1_1010_0538();
    return;
}

pub fn pass1_1040_1cb4(
    param_1: *mut Struct68,
    param_2: u32,
    param_3: Vec<u8>,
    param_4: Vec<u8>,
    param_3_00: Vec<u8>,
) {
    let mut iVar1: i32;
    let mut unaff_bp: u16;
    let mut u_var2: u16;
    let ppVar3: *mut Struct2111;
    let in_string_1: String;

    process_struct_1040_7728(
        param_1,
        (&ctx.PTR_LOOP_1050_0000 + 1),
        param_2,
        (s_You_may_not_run_a_turn__The_game_1050_00df + 9),
        param_3_00,
    );
    u_var2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    (iVar1 + 0x92) = 0;
    param_1.field_0x0 = (s_526_bmp_1050_1ee7 + 7);
    (iVar1 + 2) = &ctx.PTR_LOOP_1050_1040;
    in_string_1 = CONCAT22(unaff_bp, 2);
    ppVar3 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, in_string_1);
    (iVar1 + 0x8e) = ppVar3;
    (iVar1 + 0x90) = (ppVar3 >> 0x10);
    ppVar3 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22((in_string_1 >> 0x10), 0x37),
    );
    (iVar1 + 0x92) = ppVar3;
    (iVar1 + 0x94) = (ppVar3 >> 0x10);
    return;
}

pub fn pass1_1040_181c(
    param_1: *mut Struct68,
    param_2: u32,
    param_3: Vec<u8>,
    param_4: Vec<u8>,
    param_3_00: Vec<u8>,
) {
    let mut iVar1: i32;
    let mut unaff_bp: u16;
    let mut u_var2: u16;
    let ppVar3: *mut Struct2111;

    process_struct_1040_7728(
        param_1,
        (&ctx.PTR_LOOP_1050_0000 + 1),
        param_2,
        0xfbb,
        param_3_00,
    );
    u_var2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    (iVar1 + 0x92) = 0;
    (iVar1 + 0x94) = 0;
    param_1.field_0x0 = (s_202_flc_1050_1c46 + 2);
    (iVar1 + 2) = &ctx.PTR_LOOP_1050_1040;
    ppVar3 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_bp, 2));
    (iVar1 + 0x8e) = ppVar3;
    (iVar1 + 0x90) = (ppVar3 >> 0x10);
    return;
}

pub fn pass1_1040_123e(
    param_1: *mut Struct68,
    param_2: u32,
    param_3: Vec<u8>,
    param_4: Vec<u8>,
    param_3_00: Vec<u8>,
) -> *mut Struct68 {
    let mut iVar1: i32;
    let mut unaff_bp: u16;
    let mut u_var2: u16;
    let ppVar3: *mut Struct2111;

    process_struct_1040_7728(
        param_1,
        (&ctx.PTR_LOOP_1050_0000 + 1),
        param_2,
        0xfd1,
        param_3_00,
    );
    u_var2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    param_1.field_0x0 = 0x17b0;
    (iVar1 + 2) = &ctx.PTR_LOOP_1050_1040;
    ppVar3 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_bp, 0x46));
    (iVar1 + 0x8e) = ppVar3;
    (iVar1 + 0x90) = (ppVar3 >> 0x10);
    return param_1;
}

pub fn pass1_1040_0e1c(
    param_1: *mut Struct68,
    param_2: Vec<u8>,
    param_3: Vec<u8>,
    param_4: Vec<u8>,
) {
    let mut iVar1: i32;
    let mut unaff_bp: u16;
    let mut u_var2: u16;
    let ppVar3: *mut Struct2111;

    process_struct_1040_7728(
        param_1,
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        (s_OpWnd__getKid__Unknown_target_mo_1050_01a3 + 0x1d),
        param_4,
    );
    u_var2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    *(iVar1 + 0x92) = param_3;
    (iVar1 + 0x96) = 0;
    *(iVar1 + 0x98) = param_2;
    param_1.field_0x0 = (s_overflow_on_node__d_1050_11ca + 8);
    (iVar1 + 2) = &ctx.PTR_LOOP_1050_1040;
    ppVar3 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_bp, 0x3a));
    (iVar1 + 0x8e) = ppVar3;
    (iVar1 + 0x90) = (ppVar3 >> 0x10);
    return;
}

pub fn pass1_1040_0bfc(
    param_1: *mut Struct68,
    param_2: u32,
    param_3: Vec<u8>,
    param_4: Vec<u8>,
    param_3_00: Vec<u8>,
) -> *mut Struct68 {
    let mut iVar1: i32;
    let mut unaff_bp: u16;
    let mut u_var2: u16;
    let ppVar3: *mut Struct2111;

    process_struct_1040_7728(
        param_1,
        (&ctx.PTR_LOOP_1050_0000 + 1),
        param_2,
        0xfcd,
        param_3_00,
    );
    u_var2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    param_1.field_0x0 = 0xdb0;
    (iVar1 + 2) = &ctx.PTR_LOOP_1050_1040;
    ppVar3 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_bp, 0x39));
    (iVar1 + 0x8e) = ppVar3;
    (iVar1 + 0x90) = (ppVar3 >> 0x10);
    (iVar1 + 0x74) = 1;
    return param_1;
}

pub fn pass1_1040_0a1a(param_1: Vec<u8>) {
    let mut u_var1: i32;
    let pu_var2: *mut u32;
    let ppc_var3: fn();
    let mut u_var4: u32;
    let pu_var5: *mut u32;
    let puVar6: *mut u32;

    let struct_a: *mut Struct199;
    let ctx.dx_reg: *mut Struct199;

    let mut i_var7: i32;
    let mut i_var8: i32;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut local_18: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    u_var9 = (param_1 >> 0x10);
    i_var7 = param_1;
    u_var4 = (i_var7 + 0x8e);
    u_var10 = (u_var4 >> 0x10);
    i_var8 = u_var4;
    pu_var2 = (i_var8 + 10);
    local_6._0_2_ = pu_var2;
    pu_var5 = ((i_var8 + 0xc) | local_6);
    if (pu_var5 == 0x0) {
        return;
    }
    unsafe {
        ppc_var3 = (*pu_var2 + 0x14);
    }
    (**ppc_var3)();
    struct_a = ctx.dx_reg;
    puVar6 = pu_var5;
    if ((i_var7 + 0x70) != 0) {
        puVar6 = (i_var7 + 0x70);
        u_var1 = (i_var7 + 0x72);
        struct_a = (u_var1 | puVar6);
        if (struct_a != 0x0) {
            unsafe {
                ppc_var3 = *puVar6;
            }
            (**ppc_var3)();
            struct_a = ctx.dx_reg;
        }
    }
    struct_ops1::process_struct_1000_179c(0x14, struct_a);
    if ((struct_a | puVar6) == 0) {
        puVar6 = 0x0;
        u_var10 = 0;
    } else {
        struct_ops1::process_struct_1008_4c58(CONCAT22(struct_a, puVar6));
        u_var10 = ctx.dx_reg;
    }
    (i_var7 + 0x70) = puVar6;
    (i_var7 + 0x72) = u_var10;
    pass1_1008_4d84((i_var7 + 0x70), pu_var5);
    return;
}

pub fn pass1_1040_06e8(
    param_1: *mut Struct68,
    param_2: u32,
    param_3: Vec<u8>,
    param_4: Vec<u8>,
    param_3_00: Vec<u8>,
) -> *mut Struct68 {
    let mut iVar1: i32;
    let mut unaff_bp: u16;
    let mut u_var2: u16;
    let ppVar3: *mut Struct2111;

    process_struct_1040_7728(
        param_1,
        (&ctx.PTR_LOOP_1050_0000 + 1),
        param_2,
        0xfbc,
        param_3_00,
    );
    u_var2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    param_1.field_0x0 = 0xb90;
    (iVar1 + 2) = &ctx.PTR_LOOP_1050_1040;
    ppVar3 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_bp, 7));
    (iVar1 + 0x8e) = ppVar3;
    (iVar1 + 0x90) = (ppVar3 >> 0x10);
    return param_1;
}

pub fn pass1_1038_eeda(param_1: *mut Struct68, param_2: Vec<u8>) {
    let mut iVar1: i32;
    let mut unaff_bp: u16;
    let mut u_var2: u16;
    let ppVar3: *mut Struct2111;

    process_struct_1040_7728(
        param_1,
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        (s_notepad_read_me_1050_0162 + 4),
        param_2,
    );
    u_var2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    (iVar1 + 0x92) = 0;
    (iVar1 + 0x94) = 0;
    param_1.field_0x0 = 0x67c;
    (iVar1 + 2) = &ctx.PTR_LOOP_1050_1040;
    ppVar3 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_bp, 9));
    (iVar1 + 0x8e) = ppVar3;
    (iVar1 + 0x90) = (ppVar3 >> 0x10);
    (iVar1 + 0x74) = 1;
    return;
}

pub fn pass1_1038_eb9e(param_1: *mut Struct68, param_2: Vec<u8>) -> *mut Struct68 {
    let mut u_var1: u16;

    process_struct_1040_7728(
        param_1,
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        (s_OpWnd__getKid__Unknown_target_mo_1050_01a3 + 0x24),
        param_2,
    );
    u_var1 = (param_1 >> 0x10);
    (param_1 + 0x8e) = 0;
    param_1.field_0x0 = 0xee6e;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    return param_1;
}

pub fn pass1_1038_e99a(
    param_1: *mut Struct68,
    param_2: u32,
    param_3: Vec<u8>,
    param_4: Vec<u8>,
    param_3_00: Vec<u8>,
) -> *mut Struct68 {
    let mut iVar1: i32;
    let mut unaff_bp: u16;
    let mut u_var2: u16;
    let ppVar3: *mut Struct2111;

    process_struct_1040_7728(
        param_1,
        (&ctx.PTR_LOOP_1050_0000 + 1),
        param_2,
        0xfb9,
        param_3_00,
    );
    u_var2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    param_1.field_0x0 = 0xeb32;
    (iVar1 + 2) = &PTR_LOOP_1050_1038;
    ppVar3 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_bp, 0x30));
    (iVar1 + 0x8e) = ppVar3;
    (iVar1 + 0x90) = (ppVar3 >> 0x10);
    return param_1;
}

pub fn pass1_1038_e69a(
    param_1: *mut Struct68,
    param_2: u32,
    param_3: Vec<u8>,
    param_4: Vec<u8>,
    param_3_00: Vec<u8>,
) {
    let mut iVar1: i32;
    let mut unaff_bp: u16;
    let mut u_var2: u16;
    let ppVar3: *mut Struct2111;

    process_struct_1040_7728(
        param_1,
        (&ctx.PTR_LOOP_1050_0000 + 1),
        param_2,
        0xfcb,
        param_3_00,
    );
    u_var2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    (iVar1 + 0x92) = 0;
    param_1.field_0x0 = 0xe92e;
    (iVar1 + 2) = &PTR_LOOP_1050_1038;
    ppVar3 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_bp, 0x43));
    (iVar1 + 0x8e) = ppVar3;
    (iVar1 + 0x90) = (ppVar3 >> 0x10);
    return;
}

pub fn pass1_1038_e2d0(param_1: *mut Struct68, param_2: Vec<u8>) -> *mut Struct68 {
    let mut u_var1: u16;

    process_struct_1040_7728(
        param_1,
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        (s_OpWnd__getKid__Unknown_target_mo_1050_01a3 + 0x20),
        param_2,
    );
    u_var1 = (param_1 >> 0x10);
    (param_1 + 0x8e) = 0;
    param_1.field_0x0 = 0xe62e;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    return param_1;
}

pub fn pass1_1038_e140(
    param_1: *mut Struct68,
    param_2: u32,
    param_3: Vec<u8>,
    param_4: Vec<u8>,
    param_3_00: Vec<u8>,
) -> *mut Struct68 {
    process_struct_1040_7728(
        param_1,
        (&ctx.PTR_LOOP_1050_0000 + 1),
        param_2,
        0xfc2,
        param_3_00,
    );
    param_1.field_0x0 = 0xe264;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    return param_1;
}

pub fn pass1_1038_df86(param_1: Vec<u8>) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let BVar3: bool;
    let b_var4: bool;
    let pu_var5: *mut u16;
    let struct_a: *mut Struct199;
    let paVar6: *mut Struct199;
    let mut u_var7: i32;
    let mut i_var8: i32;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let pp_var11: *mut Struct2111;
    let mut in_stack_0000ffe6: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    pp_var11 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000ffe6, 2));
    u_var1 = (pp_var11 + 0x68);
    u_var9 = (param_1 >> 0x10);
    i_var8 = param_1;
    BVar3 = pass1_1010_041a();
    if (BVar3 != 0) {
        pass1_1010_038e((i_var8 + 0x92), 1);
        pass1_1038_af40(ctx._g_Struct112_a, *(i_var8 + 8), 0x1e);
        return;
    }
    load_string_1010_847e(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x7d5,
    );
    u_var10 = 0x1000;
    paVar6 = struct_a;
    b_var4 = BVar3;
    struct_ops1::process_struct_1000_179c(0xb4, struct_a);
    u_var7 = paVar6 | b_var4;
    if (u_var7 == 0) {
        u_var9 = 0;
        u_var7 = 0;
    } else {
        u_var10 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
        pu_var5 = process_struct_1040_8478(
            CONCAT22(paVar6, b_var4),
            0x20,
            u_var1,
            CONCAT22(struct_a, BVar3),
            (i_var8 + 6),
        );
        u_var9 = SUB42(pu_var5, 0);
    }
    _local_16 = CONCAT22(u_var7, u_var9);
    ppc_var2 = (*_local_16 + 0x74);
    ppc_var2(u_var10, u_var9, u_var7);
    return;
}

pub fn pass1_1038_d756(param_1: *mut Struct68, param_2: Vec<u8>) -> *mut Struct68 {
    let pp_var1: fn();
    let mut i_var2: i32;
    let mut unaff_bp: u16;
    let mut u_var3: u16;
    let ppVar4: *mut Struct2111;

    process_struct_1040_7728(
        param_1,
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        (s_New_failed_in_Op__Op__Simulator_1050_0110 + 0xb),
        param_2,
    );
    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    (i_var2 + 0x8e) = 0;
    (i_var2 + 0x90) = 0;
    (i_var2 + 0x92) = 0;
    (i_var2 + 0x96) = 0;
    param_1.field_0x0 = 0xe0d4;
    (i_var2 + 2) = &PTR_LOOP_1050_1038;
    ppVar4 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_bp, 0x2b));
    (i_var2 + 0x92) = ppVar4;
    (i_var2 + 0x94) = (ppVar4 >> 0x10);
    pp_var1 = ((i_var2 + 0x92) + 4);
    (**pp_var1)();
    return param_1;
}

pub fn pass1_1038_d242(param_1: *mut Struct68, param_2: Vec<u8>) -> *mut Struct68 {
    let mut u_var1: u16;

    process_struct_1040_7728(
        param_1,
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        (ctx.s_New_failed_in_Op__Op__Simulator_1050_0130 + 0xe),
        param_2,
    );
    u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xd6ea;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    (param_1 + 0x74) = 1;
    return param_1;
}

pub fn pass1_1038_cd06(
    param_1: *mut Struct68,
    param_2: u32,
    param_3: Vec<u8>,
    param_4: Vec<u8>,
    param_3_00: Vec<u8>,
) {
    let mut iVar1: i32;
    let mut unaff_bp: u16;
    let mut u_var2: u16;
    let ppVar3: *mut Struct2111;

    process_struct_1040_7728(
        param_1,
        (&ctx.PTR_LOOP_1050_0000 + 1),
        param_2,
        0xfcc,
        param_3_00,
    );
    u_var2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    (iVar1 + 0x92) = 0;
    param_1.field_0x0 = 0xcf00;
    (iVar1 + 2) = &PTR_LOOP_1050_1038;
    ppVar3 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_bp, 0x42));
    (iVar1 + 0x8e) = ppVar3;
    (iVar1 + 0x90) = (ppVar3 >> 0x10);
    return;
}

pub fn pass1_1038_cad8(param_1: u32, param_2: Vec<u8>) -> u8 {
    let mut iVar1: i32;
    let mut unaff_bp: u16;
    let mut u_var2: u16;
    let ppVar3: *mut Struct2111;

    process_struct_1040_7728(
        param_1,
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        (s_Unsupported_FileStructType_in_Op_1050_01ca + 1),
        param_2,
    );
    u_var2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    param_1 = 0xcc9a;
    (iVar1 + 2) = &PTR_LOOP_1050_1038;
    ppVar3 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_bp, 0x2c));
    (iVar1 + 0x8e) = ppVar3;
    (iVar1 + 0x90) = (ppVar3 >> 0x10);
    (iVar1 + 0x74) = 0;
    return param_1;
}

pub fn pass1_1038_c7b8(
    param_1: *mut Struct68,
    param_2: u32,
    param_3: Vec<u8>,
    param_4: Vec<u8>,
    param_3_00: Vec<u8>,
) -> *mut Struct68 {
    let mut iVar1: i32;
    let mut unaff_bp: u16;
    let mut u_var2: u16;
    let ppVar3: *mut Struct2111;

    process_struct_1040_7728(
        param_1,
        (&ctx.PTR_LOOP_1050_0000 + 1),
        param_2,
        0xfb8,
        param_3_00,
    );
    u_var2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    param_1.field_0x0 = 0xca6c;
    (iVar1 + 2) = &PTR_LOOP_1050_1038;
    ppVar3 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_bp, 5));
    (iVar1 + 0x8e) = ppVar3;
    (iVar1 + 0x90) = (ppVar3 >> 0x10);
    return param_1;
}

pub fn pass1_1038_c4a2(
    param_1: *mut Struct68,
    param_2: u32,
    param_3: Vec<u8>,
    param_4: Vec<u8>,
    param_3_00: Vec<u8>,
) {
    let mut iVar1: i32;
    let mut unaff_bp: u16;
    let mut u_var2: u16;
    let ppVar3: *mut Struct2111;

    process_struct_1040_7728(
        param_1,
        (&ctx.PTR_LOOP_1050_0000 + 1),
        param_2,
        (s_You_may_not_run_a_turn__The_game_1050_0172 + 10),
        param_3_00,
    );
    u_var2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    (iVar1 + 0x92) = 0;
    (iVar1 + 0x96) = 0;
    param_1.field_0x0 = 0xc74c;
    (iVar1 + 2) = &PTR_LOOP_1050_1038;
    ppVar3 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_bp, 0x3b));
    (iVar1 + 0x8e) = ppVar3;
    (iVar1 + 0x90) = (ppVar3 >> 0x10);
    return;
}

pub fn pass1_1038_bddc(
    param_1: *mut Struct68,
    param_2: u32,
    param_3: Vec<u8>,
    param_4: Vec<u8>,
    param_3_00: Vec<u8>,
) {
    let mut iVar1: i32;
    let mut unaff_bp: u16;
    let mut u_var2: u16;
    let ppVar3: *mut Struct2111;

    process_struct_1040_7728(
        param_1,
        (&ctx.PTR_LOOP_1050_0000 + 1),
        param_2,
        (s_You_may_not_run_a_turn__The_game_1050_0172 + 4),
        param_3_00,
    );
    u_var2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    (iVar1 + 0x92) = 0;
    (iVar1 + 0x94) = 0;
    (iVar1 + 0x96) = 0;
    (iVar1 + 0x98) = 0;
    (iVar1 + 0x9a) = 0;
    (iVar1 + 0x9c) = 0;
    param_1.field_0x0 = 0xc436;
    (iVar1 + 2) = &PTR_LOOP_1050_1038;
    ppVar3 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_bp, 0x3b));
    (iVar1 + 0x8e) = ppVar3;
    (iVar1 + 0x90) = (ppVar3 >> 0x10);
    return;
}

pub fn pass1_1038_b772(param_1: *mut Struct68) {
    let mut iVar1: i32;
    let mut unaff_bp: u16;
    let mut u_var2: u16;
    let ppVar3: *mut Struct2111;
    let in_stack_00000010: Vec<u8>;
    let in_string_1: String;

    process_struct_1040_7728(
        param_1,
        (s_New_failed_in_Op__Op__Simulator_1050_0097 + 3),
        0,
        0xfbf,
        in_stack_00000010,
    );
    u_var2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    (iVar1 + 0x92) = 0;
    (iVar1 + 0x96) = 1;
    (iVar1 + 0x98) = 0;
    param_1.field_0x0 = 0xbd70;
    (iVar1 + 2) = &PTR_LOOP_1050_1038;
    in_string_1 = CONCAT22(unaff_bp, 0x36);
    ppVar3 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, in_string_1);
    (iVar1 + 0x8e) = ppVar3;
    (iVar1 + 0x90) = (ppVar3 >> 0x10);
    ppVar3 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22((in_string_1 >> 0x10), 6),
    );
    (iVar1 + 0x92) = ppVar3;
    (iVar1 + 0x94) = (ppVar3 >> 0x10);
    return;
}

pub fn pass1_1038_ab82(param_1: *mut Struct68, param_2: Vec<u8>) -> *mut Struct68 {
    process_struct_1040_7728(param_1, (&ctx.PTR_LOOP_1050_0000 + 1), 0, 0xfd3, param_2);
    param_1.field_0x0 = 0xad72;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    return param_1;
}

pub fn pass1_1038_a122(
    param_1: i32,
    param_2: Vec<u8>,
    param_3: Vec<u8>,
    param_3_00: u32,
    param_5: u32,
) -> *mut Vec<u8> {
    let local_e: Vec<u8>;
    let puStack6: Vec<u8>;
    let puStack4: Vec<u8>;

    puStack6 = param_5;
    puStack4 = (param_5 >> 0x10);
    process_struct_1040_7728(
        CONCAT22(param_2, param_1),
        param_3,
        param_3_00,
        puStack6,
        puStack4,
    );
    (param_1 + 0x8e) = 0;
    *CONCAT22(param_2, param_1) = 0xa2d0;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1038_9f76(
    param_1: *mut Struct68,
    param_2: u32,
    param_3: Vec<u8>,
    param_4: Vec<u8>,
    param_3_00: Vec<u8>,
) -> *mut Struct68 {
    process_struct_1040_7728(
        param_1,
        (&ctx.PTR_LOOP_1050_0000 + 1),
        param_2,
        0xfba,
        param_3_00,
    );
    param_1.field_0x0 = 0xa0b6;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    return param_1;
}

pub fn pass1_1038_9144(ctx: &mut AppContext, param_1: Vec<u8>, param_2: Vec<u8>) {
    let mut u_var1: u32;
    let mut u_var2: i32;
    let struct_a: *mut Struct199;
    let struct_a_00: *mut Struct199;
    let mut i_var3: i32;
    let mut i_var4: i32;
    let mut unaff_si: u16;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let pp_var7: *mut Struct2111;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    struct_ops1::process_struct_1040_b082(param_1, CONCAT22(param_2, 0xfaa));
    u_var5 = (param_1 >> 0x10);
    i_var3 = param_1;
    (i_var3 + 0x94) = 0;
    (i_var3 + 0x96) = 0;
    (i_var3 + 0x98) = 0;
    param_1 = 0x99a2;
    (i_var3 + 2) = &PTR_LOOP_1050_1038;
    (i_var3 + 0x8a) = 0x27;
    pp_var7 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x28));
    struct_a = (pp_var7 >> 0x10);
    u_var2 = pp_var7;
    (i_var3 + 0x98) = u_var2;
    (i_var3 + 0x9a) = struct_a;
    struct_ops1::process_struct_1000_179c(0x18, struct_a);
    struct_a_00 = (struct_a | u_var2);
    if (struct_a_00 == 0x0) {
        (i_var3 + 0x90) = 0;
    } else {
        process_struct_1040_a598(CONCAT22(struct_a, u_var2));
        (i_var3 + 0x90) = u_var2;
        (i_var3 + 0x92) = ctx.dx_reg;
        struct_a_00 = ctx.dx_reg;
    }
    (i_var3 + 0x90) = 0x11;
    i_var4 = (i_var3 + 0x90);
    u_var2 = i_var4 * 10 + 2;
    struct_ops1::process_struct_1000_179c(u_var2, struct_a_00);
    _local_8 = CONCAT22(struct_a_00, u_var2);
    if ((struct_a_00 | u_var2) == 0) {
        u_var1 = (i_var3 + 0x90);
        (u_var1 + 2) = 0;
    } else {
        *_local_8 = i_var4;
        call_fn_ptr_1000_5586(
            0xa564,
            &ctx.PTR_LOOP_1050_1040,
            i_var4,
            10,
            u_var2 + 2,
            struct_a_00,
        );
        u_var1 = (i_var3 + 0x90);
        u_var6 = (u_var1 >> 0x10);
        i_var4 = u_var1;
        (i_var4 + 2) = u_var2 + 2;
        (i_var4 + 4) = struct_a_00;
    }
    u_var1 = (i_var3 + 0x90);
    (u_var1 + 10) = 0x18;
    u_var1 = (i_var3 + 0x90);
    (u_var1 + 0x12) = (i_var3 + 10);
    return;
}

pub unsafe fn pass1_1038_9124(param_1: &mut Struct44, param_2: Vec<u8>) {
    let pu8_var1: Vec<u8>;
    let mut b_var2: u8;
    let mut cVar3: u8;
    let mut u_var4: u32;
    let mut in_AL: u8;
    let mut u_var5: i32;
    let mut bVar6: u8;
    let mut in_CL: u8;
    let mut in_dx: i32;
    let mut bVar7: u8;
    let struct_a: *mut Struct199;
    let struct_a_00: *mut Struct199;

    let mut in_bx: i32;
    let mut bVar10: u8;
    let mut i_var8: i32;
    let mut i_var9: i32;
    let pu_var11: *mut u16;
    let unaff_bp: *mut u16;
    let unaff_si: Vec<u8>;
    let mut u_var12: u16;
    let mut u_var13: u16;
    let mut unaff_ss: u16;
    let mut in_CF: u8;
    let mut b_var14: bool;
    let mut bVar15: bool;
    let ppVar16: *mut Struct2111;
    let local_4e: u8;
    let pi_stack8: *mut i32;

    pu_var11 = &stack0xfffe;
    cVar3 = '\x0f';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var11 = pu_var11 + -1;
        unsafe {
            *pu_var11 = *unaff_bp;
        }
        cVar3 = cVar3 + -1;
        '\0' < cVar3
    } {}
    bVar10 = (in_bx >> 8);
    unaff_si[in_bx] = bVar10;
    bVar7 = ((in_dx + 1) >> 8);
    b_var14 = CARRY1(bVar7, bVar10) || CARRY1(bVar7 + bVar10, in_CF);
    bVar6 = (in_dx + 1);
    pu8_var1 = unaff_si + in_bx;
    unsafe {
        bVar7 = *pu8_var1;
        b_var2 = *pu8_var1 - bVar6;
        bVar15 = *pu8_var1 < bVar6 || b_var2 < b_var14;
        *pu8_var1 = b_var2 - b_var14;
        if (*pu8_var1 != 0
            && (SBORROW1(bVar7, bVar6) != SBORROW1(b_var2, b_var14)) == (*pu8_var1 < '\0'))
        {
            pu8_var1 = unaff_si;
            bVar7 = *pu8_var1;
            b_var2 = *pu8_var1;
            *pu8_var1 = b_var2 + bVar10 + bVar15;
            b_var14 = CARRY1(local_4e, in_bx)
                || CARRY1(
                    local_4e + in_bx,
                    CARRY1(bVar7, bVar10) || CARRY1(b_var2 + bVar10, bVar15),
                );
            pu8_var1 = unaff_si + -0x4f;
            bVar15 = CARRY1(*pu8_var1, bVar10) || CARRY1(*pu8_var1 + bVar10, b_var14);
            *pu8_var1 = *pu8_var1 + bVar10 + b_var14;
            u_var5 = CONCAT11(0x40, in_AL + 0x1) + 2;
            pu8_var1 = &stack0xfffe + unaff_si;
            bVar7 = *pu8_var1;
            bVar6 = (u_var5 >> 8);
            b_var2 = *pu8_var1 + bVar6;
            *pu8_var1 = b_var2 + bVar15;
            pu8_var1 = unaff_si + in_bx;
            *pu8_var1 =
                *pu8_var1 + u_var5 + in_CL + (CARRY1(bVar7, bVar6) || CARRY1(b_var2, bVar15));
            struct_ops1::process_struct_1040_b082(param_1, CONCAT22(param_2, 0xfaa));
            u_var12 = (param_1 >> 0x10);
            i_var8 = param_1;
            (i_var8 + 0x94) = 0;
            (i_var8 + 0x96) = 0;
            (i_var8 + 0x98) = 0;
            param_1.ptr_a_lo = 0x99a2;
            (i_var8 + 2) = &PTR_LOOP_1050_1038;
            (i_var8 + 0x8a) = 0x27;
            ppVar16 =
                process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x28));
            struct_a = (ppVar16 >> 0x10);
            u_var5 = ppVar16;
            (i_var8 + 0x98) = u_var5;
            (i_var8 + 0x9a) = struct_a;
            struct_ops1::process_struct_1000_179c(0x18, struct_a);
            struct_a_00 = (struct_a | u_var5);
            if (struct_a_00 == 0x0) {
                (i_var8 + 0x90) = 0;
            } else {
                process_struct_1040_a598(CONCAT22(struct_a, u_var5));
                (i_var8 + 0x90) = u_var5;
                (i_var8 + 0x92) = ctx.dx_reg;
                struct_a_00 = ctx.dx_reg;
            }
            (i_var8 + 0x90) = 0x11;
            i_var9 = (i_var8 + 0x90);
            u_var5 = i_var9 * 10 + 2;
            struct_ops1::process_struct_1000_179c(u_var5, struct_a_00);
            pi_stack8 = CONCAT22(struct_a_00, u_var5);
            if ((struct_a_00 | u_var5) == 0) {
                u_var4 = (i_var8 + 0x90);
                (u_var4 + 2) = 0;
            } else {
                *pi_stack8 = i_var9;
                call_fn_ptr_1000_5586(
                    0xa564,
                    &ctx.PTR_LOOP_1050_1040,
                    i_var9,
                    10,
                    u_var5 + 2,
                    struct_a_00,
                );
                u_var4 = (i_var8 + 0x90);
                u_var13 = (u_var4 >> 0x10);
                i_var9 = u_var4;
                (i_var9 + 2) = u_var5 + 2;
                (i_var9 + 4) = struct_a_00;
            }
            u_var4 = (i_var8 + 0x90);
            (u_var4 + 10) = 0x18;
            u_var4 = (i_var8 + 0x90);
            (u_var4 + 0x12) = (i_var8 + 10);
            return;
        }
    }
    if (unsafe { *pu8_var1 != 0 }) {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1038_8caa(param_1: *mut Vec<u8>, param_2: Vec<u8>) -> *mut Vec<u8> {
    let mut iVar1: i32;
    let mut unaff_bp: u16;
    let mut u_var2: u16;
    let ppVar3: *mut Struct2111;

    struct_ops1::process_struct_1040_b082(param_1, CONCAT22(param_2, 0x185a));
    u_var2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x94) = 0;
    unsafe {
        *param_1 = 0x90c8;
    }
    (iVar1 + 2) = &PTR_LOOP_1050_1038;
    ppVar3 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_bp, 0x3f));
    (iVar1 + 0x94) = ppVar3;
    (iVar1 + 0x96) = (ppVar3 >> 0x10);
    return param_1;
}

pub fn pass1_1038_88f2(param_1: *mut Vec<u8>, param_2: Vec<u8>) {
    let mut iVar1: i32;
    let mut u_var2: u16;

    struct_ops1::process_struct_1040_b082(param_1, CONCAT22(param_2, 0x184c));
    u_var2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x94) = _ctx.PTR_LOOP_1050_5a68;
    (iVar1 + 0x98) = 0;
    (iVar1 + 0x9a) = 0;
    (iVar1 + 0x9c) = 0;
    (iVar1 + 0x9e) = 0;
    unsafe {
        *param_1 = 0x8c2e;
    }
    (iVar1 + 2) = &PTR_LOOP_1050_1038;
    return;
}

pub fn pass1_1038_88d2(param_1: &mut Struct44) {
    let pu8_var1: Vec<u8>;
    let mut b_var2: u8;
    let mut cVar3: u8;
    let mut u8_var4: u8;
    let mut in_CL: u8;
    let mut in_dx: i32;
    let mut b_var5: u8;
    let mut in_bx: i32;
    let mut bVar7: u8;
    let mut i32_var6: i32;
    let puVar8: *mut u16;
    let unaff_bp: *mut u16;
    let unaff_si: Vec<u8>;
    let mut u_var9: u16;
    let mut unaff_ss: u16;
    let mut in_CF: u8;
    let mut bVar10: bool;
    let mut bVar11: bool;
    let pu_var12: *mut u16;
    let local_4e: u8;
    let puStack34: Vec<u8>;

    puStack34 = &stack0xfffe;
    puVar8 = &stack0xfffe;
    cVar3 = '\x0f';
    while {
        unaff_bp = unaff_bp + -1;
        puVar8 = puVar8 + -1;
        unsafe {
            *puVar8 = *unaff_bp;
        }
        cVar3 = cVar3 + -1;
        '\0' < cVar3
    } {}
    bVar7 = (in_bx >> 8);
    unaff_si[in_bx] = bVar7;
    b_var5 = ((in_dx + 1) >> 8);
    bVar10 = CARRY1(b_var5, bVar7) || CARRY1(b_var5 + bVar7, in_CF);
    u8_var4 = (in_dx + 1);
    pu_var12 = CONCAT22(&stack0xbf2a, &stack0xfffe);
    pu8_var1 = unaff_si + in_bx;
    unsafe {
        b_var5 = *pu8_var1;
        b_var2 = *pu8_var1 - u8_var4;
        bVar11 = *pu8_var1 < u8_var4 || b_var2 < bVar10;
        *pu8_var1 = b_var2 - bVar10;
        if (*pu8_var1 != 0
            && (SBORROW1(b_var5, u8_var4) != SBORROW1(b_var2, bVar10)) == (*pu8_var1 < '\0'))
        {
            pu8_var1 = unaff_si;
            bVar10 = CARRY1(*pu8_var1, bVar7) || CARRY1(*pu8_var1 + bVar7, bVar11);
            *pu8_var1 = *pu8_var1 + bVar7 + bVar11;
            b_var5 = local_4e + in_bx;
            bVar11 = CARRY1(local_4e, in_bx) || CARRY1(b_var5, bVar10);
            local_4e = b_var5 + bVar10;
            pu8_var1 = unaff_si + -0x4f;
            b_var5 = *pu8_var1;
            b_var2 = *pu8_var1;
            *pu8_var1 = b_var2 + bVar7 + bVar11;
            pu8_var1 = unaff_si + -0x78;
            *pu8_var1 =
                *pu8_var1 + in_CL + (CARRY1(b_var5, bVar7) || CARRY1(b_var2 + bVar7, bVar11));
            puStack34 = &stack0xfffe;
            struct_ops1::process_struct_1040_b082(pu_var12, CONCAT22(in_bx, 0x184c));
            u_var9 = (pu_var12 >> 0x10);
            i32_var6 = pu_var12;
            (i32_var6 + 0x94) = _ctx.PTR_LOOP_1050_5a68;
            (i32_var6 + 0x98) = 0;
            (i32_var6 + 0x9a) = 0;
            (i32_var6 + 0x9c) = 0;
            (i32_var6 + 0x9e) = 0;
            *pu_var12 = 0x8c2e;
            (i32_var6 + 2) = &PTR_LOOP_1050_1038;
            return;
        }
        if (*pu8_var1 != 0) {
            error_check_1000_17ce(param_1);
        }
    }
    return;
}

pub fn pass1_1038_7d10(param_1: u32, param_2: Vec<u8>) {
    let local_bx_18: *mut Struct1172;
    let mut unaff_bp: u16;
    let mut u_var1: u16;
    let struct_a: *mut Struct2111;

    struct_ops1::process_struct_1040_b082(param_1, CONCAT22(param_2, 0x1853));
    u_var1 = (param_1 >> 0x10);
    local_bx_18 = param_1;
    local_bx_18.field_0x94 = 0x0;
    param_1 = 0x8876;
    local_bx_18.field_0x2 = &PTR_LOOP_1050_1038;
    struct_a = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_bp, 0x40));
    &local_bx_18.field_0x94 = struct_a;
    (&local_bx_18.field_0x94 + 2) = (struct_a >> 0x10);
    return param_1;
}

pub fn pass1_1038_78e2(param_1: *mut Vec<u8>) {
    let paVar1: *mut Struct199;
    let in_dx: *mut Struct199;
    let struct_a: *mut Struct199;
    let mut u_var2: i32;
    let local_bx_4: *mut Struct1169;
    let mut u_var3: u16;
    let mut local_4: u16;

    u_var3 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    paVar1 = 0x0;
    unsafe {
        *param_1 = 0x0;
    }
    &local_bx_4.field_0x4 = 0;
    _PTR_LOOP_1050_5a64 = param_1;
    struct_ops1::process_struct_1000_179c(0xc, in_dx);
    struct_a = (in_dx | paVar1);
    if (struct_a == 0x0) {
        unsafe {
            *param_1 = 0x0;
        }
    } else {
        paVar1 = struct_ops1::process_struct_1008_574a(CONCAT22(in_dx, paVar1));
        param_1 = paVar1;
        local_bx_4.field_0x2 = struct_a;
    }
    struct_ops1::process_struct_1000_179c(0xc, struct_a);
    u_var2 = struct_a | paVar1;
    if (u_var2 == 0) {
        paVar1 = 0x0;
        u_var2 = 0;
    } else {
        paVar1 = struct_ops1::process_struct_1008_574a(CONCAT22(struct_a, paVar1));
    }
    local_bx_4.field_0x4 = paVar1;
    local_bx_4.field_0x6 = u_var2;
    return;
}

pub fn pass1_1038_6520(param_1: *mut Struct1144) {
    let local_bx_4: *mut Struct1144;
    let mut local_es_4: u16;

    local_es_4 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    param_1.field_0x0 = ctx.s_1_1050_389a;
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    local_bx_4.field_0x4 = 0x0;
    local_bx_4.field_0x8 = 0x0;
    local_bx_4.field_0xc = 0;
    local_bx_4.field_0xe = 0x0;
    local_bx_4.field_0x12 = 0;
    local_bx_4.field_0x14 = 0;
    local_bx_4.field_0x16 = 0;
    zero_list_1008_3e38((param_1 & 0xffff0000 | &local_bx_4.field_0x1a));
    local_bx_4.field_0x20 = 0;
    local_bx_4.field_0x24 = 0;
    local_bx_4.field_0x26 = 0;
    local_bx_4.field_0x28 = 0;
    param_1.field_0x0 = 0x78de;
    local_bx_4.field_0x2 = &PTR_LOOP_1050_1038;
    return;
}

pub fn pass1_1038_2b9a(param_1: Vec<u8>) {
    let pu_var1: *mut u32;
    let pu_var2: *mut u32;
    let mut in_ax: i32;
    let pu_var3: *mut u32;
    let mut i_var4: i32;
    let in_dx: *mut Struct199;
    let mut i_var5: i32;
    let puVar6: *mut u32;
    let mut u_var7: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    struct_ops1::process_struct_1000_179c(0x118, in_dx);
    _local_a = CONCAT22(in_dx, in_ax);
    i_var5 = param_1;
    u_var7 = (param_1 >> 0x10);
    if ((in_dx | in_ax) != 0) {
        *_local_a = ctx.s_1_1050_389a;
        (in_ax + 2) = &ctx.PTR_LOOP_1050_1008;
        (in_ax + 4) = (i_var5 + 4);
        pu_var3 = (i_var5 + 8);
        puVar6 = (in_ax + 8);
        i_var4 = 0x40;
        while (i_var4 != 0) {
            i_var4 = i_var4 + -1;
            pu_var2 = puVar6;
            puVar6 = puVar6 + 1;
            pu_var1 = pu_var3;
            pu_var3 = pu_var3 + 1;
            unsafe {
                *pu_var2 = *pu_var1;
            }
        }
        *_local_a = 0x6ad2;
        (in_ax + 2) = &PTR_LOOP_1050_1028;
        (in_ax + 0x108) = (i_var5 + 0x108);
        (in_ax + 0x10c) = (i_var5 + 0x10c);
        (in_ax + 0x110) = (i_var5 + 0x110);
        (in_ax + 0x114) = (i_var5 + 0x114);
        *_local_a = 0x309a;
        (in_ax + 2) = &PTR_LOOP_1050_1038;
    }
    (i_var5 + 0x114) = 0;
    (i_var5 + 0x110) = 0;
    return;
}

pub fn pass1_1038_2944(param_1: u32) {
    let paVar1: *mut Struct1074;
    let paVar2: *mut Struct1073;
    let ctx.ax_reg: *mut Struct1071;
    let local_CX_80: *mut Struct1072;
    let in_dx: *mut Struct199;
    let local_SI_80: *mut Struct1074;
    let local_DI_80: *mut Struct1073;
    let mut u_var3: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    struct_ops1::process_struct_1000_179c(0x108, in_dx);
    _local_a = CONCAT22(in_dx, ctx.ax_reg);
    if ((in_dx | ctx.ax_reg) != 0) {
        *_local_a = ctx.s_1_1050_389a;
        ctx.ax_reg.field_0x2 = &ctx.PTR_LOOP_1050_1008;
        u_var3 = (param_1 >> 0x10);
        ctx.ax_reg.field_0x4 = (param_1 + 4);
        local_SI_80 = (param_1 + 8);
        local_DI_80 = &ctx.ax_reg.field_0x8;
        local_CX_80 = (s_New_failed_in_Op__Op__ResLibr_1050_0035 + 0xb);
        while (local_CX_80 != 0x0) {
            local_CX_80 = local_CX_80 + -1;
            paVar2 = local_DI_80;
            local_DI_80 = &local_DI_80.field_0x4;
            paVar1 = local_SI_80;
            local_SI_80 = &local_SI_80.field_0x4;
            paVar2.field_0x0 = paVar1.field_0x0;
        }
        *_local_a = 0x6ad2;
        ctx.ax_reg.field_0x2 = &PTR_LOOP_1050_1028;
        *_local_a = (s_fem110_wav_1050_29fa + 4);
        ctx.ax_reg.field_0x2 = &PTR_LOOP_1050_1038;
    }
    return;
}

pub fn pass1_1038_0cf0(param_1: *mut Struct1054) {
    let paVar1: *mut Struct1057;
    let paVar2: *mut Struct1056;
    let ctx.ax_reg: *mut Struct1055;
    let mut i_var3: i32;
    let in_dx: *mut Struct199;
    let local_bx_41: *mut Struct1054;
    let local_SI_80: *mut Struct1057;
    let local_DI_80: *mut Struct1056;
    let local_es_41: *mut Struct1054;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    struct_ops1::process_struct_1000_179c(0x10c, in_dx);
    _local_a = CONCAT22(in_dx, ctx.ax_reg);
    if ((in_dx | ctx.ax_reg) != 0) {
        *_local_a = ctx.s_1_1050_389a;
        ctx.ax_reg.field_0x2 = &ctx.PTR_LOOP_1050_1008;
        local_es_41 = (param_1 >> 0x10);
        local_bx_41 = param_1;
        ctx.ax_reg.field_0x4 = local_bx_41.field_0x4;
        local_SI_80 = &local_bx_41.field_0x8;
        local_DI_80 = &ctx.ax_reg.field_0x8;
        i_var3 = 0x40;
        while (i_var3 != 0) {
            i_var3 = i_var3 + -1;
            paVar2 = local_DI_80;
            local_DI_80 = &local_DI_80.field_0x4;
            paVar1 = local_SI_80;
            local_SI_80 = &local_SI_80.field_0x4;
            paVar2.field_0x0 = paVar1.field_0x0;
        }
        *_local_a = 0x6ad2;
        ctx.ax_reg.field_0x2 = &PTR_LOOP_1050_1028;
        ctx.ax_reg.field_0x108 = local_bx_41.field_0x108;
        *_local_a = s_198_flc_1050_1c2e;
        ctx.ax_reg.field_0x2 = &PTR_LOOP_1050_1038;
    }
    return;
}

pub fn pass1_1038_0000(param_1: Vec<u8>) {
    let paVar1: *mut Struct1049;
    let paVar2: *mut Struct1048;
    let ctx.ax_reg: *mut Struct1046;
    let local_CX_80: *mut Struct1047;
    let in_dx: *mut Struct199;
    let local_SI_80: *mut Struct1049;
    let local_DI_80: *mut Struct1048;
    let mut u_var3: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    // Segment:    8
    // Offset:     000606c0
    // Length:     ef91
    // Min Alloc:  ef91
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
    struct_ops1::process_struct_1000_179c(0x108, in_dx);
    _local_a = CONCAT22(in_dx, ctx.ax_reg);
    if ((in_dx | ctx.ax_reg) != 0) {
        *_local_a = ctx.s_1_1050_389a;
        ctx.ax_reg.field_0x2 = &ctx.PTR_LOOP_1050_1008;
        u_var3 = (param_1 >> 0x10);
        ctx.ax_reg.field_0x4 = (param_1 + 4);
        local_SI_80 = (param_1 + 8);
        local_DI_80 = &ctx.ax_reg.field_0x8;
        local_CX_80 = (s_New_failed_in_Op__Op__ResLibr_1050_0035 + 0xb);
        while (local_CX_80 != 0x0) {
            local_CX_80 = local_CX_80 + -1;
            paVar2 = local_DI_80;
            local_DI_80 = &local_DI_80.field_0x4;
            paVar1 = local_SI_80;
            local_SI_80 = &local_SI_80.field_0x4;
            paVar2.field_0x0 = paVar1.field_0x0;
        }
        *_local_a = 0x6ad2;
        ctx.ax_reg.field_0x2 = &PTR_LOOP_1050_1028;
        *_local_a = 0xb96;
        ctx.ax_reg.field_0x2 = &PTR_LOOP_1050_1038;
    }
    return;
}

pub fn pass1_1030_ebf8(param_1: Vec<u8>) {
    let paVar1: *mut Struct1042;
    let paVar2: *mut Struct1043;
    let ctx.ax_reg: *mut Struct1044;
    let mut i_var3: i32;
    let in_dx: *mut Struct199;
    let local_SI_80: *mut Struct1042;
    let local_DI_80: *mut Struct1043;
    let mut u_var4: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    struct_ops1::process_struct_1000_179c(0x108, in_dx);
    _local_a = CONCAT22(in_dx, ctx.ax_reg);
    if ((in_dx | ctx.ax_reg) != 0) {
        *_local_a = ctx.s_1_1050_389a;
        ctx.ax_reg.field_0x2 = &ctx.PTR_LOOP_1050_1008;
        u_var4 = (param_1 >> 0x10);
        ctx.ax_reg.field_0x4 = (param_1 + 4);
        local_SI_80 = (param_1 + 8);
        local_DI_80 = &ctx.ax_reg.field_0x8;
        i_var3 = 0x40;
        while (i_var3 != 0) {
            i_var3 = i_var3 + -1;
            paVar2 = local_DI_80;
            local_DI_80 = &local_DI_80.field_0x4;
            paVar1 = local_SI_80;
            local_SI_80 = &local_SI_80.field_0x4;
            paVar2.field_0x0 = paVar1.field_0x0;
        }
        *_local_a = 0x6ad2;
        ctx.ax_reg.field_0x2 = &PTR_LOOP_1050_1028;
        *_local_a = 0xecb2;
        ctx.ax_reg.field_0x2 = 0x1030;
    }
    return;
}

pub fn pass1_1030_e98e(param_1: *mut Struct1037) {
    let paVar1: *mut Struct1041;
    let paVar2: *mut Struct1040;
    let ctx.ax_reg: *mut Struct1038;
    let local_CX_82: *mut Struct1039;
    let in_dx: *mut Struct199;
    let local_bx_43: *mut Struct1037;
    let local_SI_82: *mut Struct1041;
    let local_DI_82: *mut Struct1040;
    let local_es_43: *mut Struct1037;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    struct_ops1::process_struct_1000_179c(0x112, in_dx);
    _local_a = CONCAT22(in_dx, ctx.ax_reg);
    if ((in_dx | ctx.ax_reg) != 0) {
        *_local_a = ctx.s_1_1050_389a;
        ctx.ax_reg.field_0x2 = &ctx.PTR_LOOP_1050_1008;
        local_es_43 = (param_1 >> 0x10);
        local_bx_43 = param_1;
        ctx.ax_reg.field_0x4 = local_bx_43.field_0x4;
        local_SI_82 = &local_bx_43.field_0x8;
        local_DI_82 = &ctx.ax_reg.field_0x8;
        local_CX_82 = (s_New_failed_in_Op__Op__ResLibr_1050_0035 + 0xb);
        while (local_CX_82 != 0x0) {
            local_CX_82 = local_CX_82 + -1;
            paVar2 = local_DI_82;
            local_DI_82 = &local_DI_82.field_0x4;
            paVar1 = local_SI_82;
            local_SI_82 = &local_SI_82.field_0x4;
            paVar2.field_0x0 = paVar1.field_0x0;
        }
        *_local_a = 0x6ad2;
        ctx.ax_reg.field_0x2 = &PTR_LOOP_1050_1028;
        ctx.ax_reg.field_0x108 = local_bx_43.field_0x108;
        ctx.ax_reg.field_0x10c = local_bx_43.field_0x10c;
        ctx.ax_reg.field_0x110 = local_bx_43.field_0x110;
        *_local_a = 0xeb40;
        ctx.ax_reg.field_0x2 = 0x1030;
    }
    return;
}

pub fn pass1_1030_e7d6(param_1: Vec<u8>) {
    let paVar1: *mut Struct1034;
    let paVar2: *mut Struct1033;
    let ctx.ax_reg: *mut Struct1031;
    let local_CX_80: *mut Struct1032;
    let in_dx: *mut Struct199;
    let local_SI_80: *mut Struct1034;
    let local_DI_80: *mut Struct1033;
    let mut u_var3: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    struct_ops1::process_struct_1000_179c(0x108, in_dx);
    _local_a = CONCAT22(in_dx, ctx.ax_reg);
    if ((in_dx | ctx.ax_reg) != 0) {
        *_local_a = ctx.s_1_1050_389a;
        ctx.ax_reg.field_0x2 = &ctx.PTR_LOOP_1050_1008;
        u_var3 = (param_1 >> 0x10);
        ctx.ax_reg.field_0x4 = (param_1 + 4);
        local_SI_80 = (param_1 + 8);
        local_DI_80 = &ctx.ax_reg.field_0x8;
        local_CX_80 = (s_New_failed_in_Op__Op__ResLibr_1050_0035 + 0xb);
        while (local_CX_80 != 0x0) {
            local_CX_80 = local_CX_80 + -1;
            paVar2 = local_DI_80;
            local_DI_80 = &local_DI_80.field_0x4;
            paVar1 = local_SI_80;
            local_SI_80 = &local_SI_80.field_0x4;
            paVar2.field_0x0 = paVar1.field_0x0;
        }
        *_local_a = 0x6ad2;
        ctx.ax_reg.field_0x2 = &PTR_LOOP_1050_1028;
        *_local_a = 0xe890;
        ctx.ax_reg.field_0x2 = 0x1030;
    }
    return;
}

pub fn pass1_1030_e6c2(param_1: *mut Struct1030) {
    let pu_var1: *mut u32;
    let pu_var2: *mut u32;
    let ctx.ax_reg: *mut Struct1029;
    let mut i_var3: i32;
    let in_dx: *mut Struct199;
    let local_bx_41: *mut Struct1030;
    let pu_var4: *mut u32;
    let pu_var5: *mut u32;
    let mut u_var6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    struct_ops1::process_struct_1000_179c(0x10a, in_dx);
    _local_a = CONCAT22(in_dx, ctx.ax_reg);
    if ((in_dx | ctx.ax_reg) != 0) {
        *_local_a = ctx.s_1_1050_389a;
        ctx.ax_reg.field_0x2 = &ctx.PTR_LOOP_1050_1008;
        u_var6 = (param_1 >> 0x10);
        local_bx_41 = param_1;
        ctx.ax_reg.field_0x4 = local_bx_41.field_0x4;
        pu_var4 = &local_bx_41.field_0x8;
        pu_var5 = &ctx.ax_reg.field_0x8;
        i_var3 = 0x40;
        while (i_var3 != 0) {
            i_var3 = i_var3 + -1;
            pu_var2 = pu_var5;
            pu_var5 = pu_var5 + 1;
            pu_var1 = pu_var4;
            pu_var4 = pu_var4 + 1;
            unsafe {
                *pu_var2 = *pu_var1;
            }
        }
        *_local_a = 0x6ad2;
        ctx.ax_reg.field_0x2 = &PTR_LOOP_1050_1028;
        ctx.ax_reg.field_0x108 = local_bx_41.field_0x108;
        *_local_a = 0xe78a;
        ctx.ax_reg.field_0x2 = 0x1030;
    }
    return;
}

pub fn pass1_1030_e564(param_1: *mut Struct1026) {
    let pu_var1: *mut u32;
    let pu_var2: *mut u32;
    let ctx.ax_reg: *mut Struct1027;
    let local_CX_80: *mut Struct1025;
    let in_dx: *mut Struct199;
    let local_bx_41: *mut Struct1026;
    let pu_var3: *mut u32;
    let pu_var4: *mut u32;
    let mut u_var5: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    struct_ops1::process_struct_1000_179c(0x10c, in_dx);
    _local_a = CONCAT22(in_dx, ctx.ax_reg);
    if ((in_dx | ctx.ax_reg) != 0) {
        *_local_a = ctx.s_1_1050_389a;
        ctx.ax_reg.field_0x2 = &ctx.PTR_LOOP_1050_1008;
        u_var5 = (param_1 >> 0x10);
        local_bx_41 = param_1;
        ctx.ax_reg.field_0x4 = local_bx_41.field_0x4;
        pu_var3 = &local_bx_41.field_0x8;
        pu_var4 = &ctx.ax_reg.field_0x8;
        local_CX_80 = (s_New_failed_in_Op__Op__ResLibr_1050_0035 + 0xb);
        while (local_CX_80 != 0x0) {
            local_CX_80 = local_CX_80 + -1;
            pu_var2 = pu_var4;
            pu_var4 = pu_var4 + 1;
            pu_var1 = pu_var3;
            pu_var3 = pu_var3 + 1;
            unsafe {
                *pu_var2 = *pu_var1;
            }
        }
        *_local_a = 0x6ad2;
        ctx.ax_reg.field_0x2 = &PTR_LOOP_1050_1028;
        ctx.ax_reg.field_0x108 = local_bx_41.field_0x108;
        *_local_a = 0xe62e;
        ctx.ax_reg.field_0x2 = 0x1030;
    }
    return;
}

pub fn pass1_1030_e34e(struct_a: *mut Struct1022) {
    let pu_var1: *mut u32;
    let pu_var2: *mut u32;
    let struct_b: *mut Struct1023;
    let mut i_var3: i32;
    let struct_c: *mut Struct199;
    let struct_d: *mut Struct1022;
    let pu_var4: *mut u32;
    let pu_var5: *mut u32;
    let struct_e: *mut Struct1022;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    struct_ops1::process_struct_1000_179c(0x112, struct_c);
    _local_a = CONCAT22(struct_c, struct_b);
    if ((struct_c | struct_b) != 0) {
        *_local_a = ctx.s_1_1050_389a;
        struct_b.field_0x2 = &ctx.PTR_LOOP_1050_1008;
        struct_e = (struct_a >> 0x10);
        struct_d = struct_a;
        struct_b.field_0x4 = struct_d.field_0x4;
        pu_var4 = &struct_d.field_0x8;
        pu_var5 = &struct_b.field_0x8;
        i_var3 = 0x40;
        while (i_var3 != 0) {
            i_var3 = i_var3 + -1;
            pu_var2 = pu_var5;
            pu_var5 = pu_var5 + 1;
            pu_var1 = pu_var4;
            pu_var4 = pu_var4 + 1;
            unsafe {
                *pu_var2 = *pu_var1;
            }
        }
        *_local_a = 0x6ad2;
        struct_b.field_0x2 = &PTR_LOOP_1050_1028;
        struct_b.field_0x108 = struct_d.field_0x108;
        struct_b.field_0x10c = struct_d.field_0x10c;
        struct_b.field_0x110 = struct_d.field_0x110;
        *_local_a = 0xe4ea;
        struct_b.field_0x2 = 0x1030;
    }
    return;
}

pub fn pass1_1030_e1f4(param_1: u32) {
    let pu_var1: *mut u32;
    let pu_var2: *mut u32;
    let mut in_ax: i32;
    let pu_var3: *mut u32;
    let local_CX_80: *mut Struct1019;
    let in_dx: *mut Struct199;
    let pu_var4: *mut u32;
    let mut u_var5: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    struct_ops1::process_struct_1000_179c(0x108, in_dx);
    _local_a = CONCAT22(in_dx, in_ax);
    if ((in_dx | in_ax) != 0) {
        *_local_a = ctx.s_1_1050_389a;
        (in_ax + 2) = &ctx.PTR_LOOP_1050_1008;
        u_var5 = (param_1 >> 0x10);
        (in_ax + 4) = (param_1 + 4);
        pu_var3 = (param_1 + 8);
        pu_var4 = (in_ax + 8);
        local_CX_80 = (s_New_failed_in_Op__Op__ResLibr_1050_0035 + 0xb);
        while (local_CX_80 != 0x0) {
            local_CX_80 = local_CX_80 + -1;
            pu_var2 = pu_var4;
            pu_var4 = pu_var4 + 1;
            pu_var1 = pu_var3;
            pu_var3 = pu_var3 + 1;
            unsafe {
                *pu_var2 = *pu_var1;
            }
        }
        *_local_a = 0x6ad2;
        (in_ax + 2) = &PTR_LOOP_1050_1028;
        *_local_a = 0xe2ae;
        (in_ax + 2) = 0x1030;
    }
    return;
}

pub fn pass1_1030_8c66(
    param_1: *mut Struct104,
    param_2: u16,
    param_3: bool,
    param_4: u16,
    param_5: u32,
) {
    let mut bVar1: u8;
    let mut u_var2: i32;

    let mut local_6: u16;
    let mut local_4: u16;

    struct_ops1::process_struct_1008_4544((param_1 + 0x12));
    bVar1 = *param_3;
    u_var2 = bVar1;
    local_6 = (u_var2 + 1);
    if (0 < param_2) {
        if (u_var2 == 0) {
            local_6 = 7;
        } else {
            if (((bVar1 == 0) || (SBORROW2(u_var2, 1))) || (1 < (u_var2 - 1))) {
                local_6 = 9;
            } else {
                local_6 = 8;
            }
        }
    }
    param_5 = local_6;
    return;
}

pub fn pass1_1030_51f0(struct_a: &Struct893) -> &Struct893 {
    let local_bx_6: &Struct893;
    let mut u_var1: u16;

    u_var1 = (struct_a >> 0x10);
    local_bx_6 = struct_a;
    local_bx_6.field_0xa4 = 0;
    local_bx_6.field_0xa6 = 0;
    local_bx_6.field_0xa8 = 0;
    local_bx_6.field_0xaa = 0;
    local_bx_6.field_0xac = 0;
    return struct_a;
}

pub fn pass1_1030_4574(param_1: *mut Struct882) {
    let local_bx_6: *mut Struct882;
    let mut local_es_6: u16;

    local_es_6 = (param_1 >> 0x10);
    local_bx_6 = param_1;
    local_bx_6.field_0xc = u16_1050_518c;
    local_bx_6.field_0xe = 0x518e;
    local_bx_6.field_0x10 = &ctx.g_alloc_addr_1050_1050;
    return param_1 & 0xffff0000 | ZEXT24(&local_bx_6.field_0xc);
}

pub fn pass1_1030_44be(struct_a: *mut Struct881) {
    let struct_b: *mut Struct881;
    let struct_b_hi: *mut Struct881;
    let struct_c: *mut Struct2111;
    let mut in_stack_0000ffec: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    struct_b_hi = (struct_a >> 0x10);
    struct_b = struct_a;
    struct_a = 0;
    struct_b.field_0x8 = 0;
    struct_b.field_0x12 = 0;
    struct_b.field_0x152 = 0;
    struct_b.field_0x154 = 0;
    struct_b.field_0x156 = 0;
    struct_b.field_0x158 = 0;
    struct_b.field_0x15a = 0;
    struct_b.field_0x15c = 0;
    struct_b.field_0x160 = 0;
    struct_b.field_0x164 = 0;
    struct_b.field_0x568 = 0x0;
    struct_c = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22((in_stack_0000ffec >> 0x10), 2),
    );
    struct_b.field_0x568 = (struct_c + 100);
    return;
}

pub fn pass1_1030_3af6(
    param_1: *mut Struct880,
    param_2: u16,
    param_3: u16,
    param_4: *mut u32,
    param_5: u16,
) -> *mut Struct880 {
    let local_DI_16: *mut Struct880;
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    local_DI_16 = param_1;
    unsafe {
        param_1 = *param_4;
    }
    local_DI_16.field_0x4 = (param_4 + 1);
    local_DI_16.field_0x6 = param_3;
    local_DI_16.field_0x8 = param_2;
    return param_1;
}

pub unsafe fn process_struct_1020_7f38(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    param_1.ptr_a_lo = ctx.s_0_020_1050_3ab0;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn process_struct_1020_775a(in_struct_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    process_struct_1020_75c4(in_struct_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(in_struct_1);
    }
    return in_struct_1;
}

pub unsafe fn process_struct_1020_75c4(param_1: &mut Struct44) {
    let local_struct_1: *mut Struct376;
    let local_struct_1_hi: *mut Struct376;

    local_struct_1_hi = (param_1 >> 0x10);
    local_struct_1 = param_1;
    param_1.ptr_a_lo = 0x7780;
    local_struct_1.ptr_a_hi = 0x1020;
    local_struct_1.offset_xe2 = 0x781c;
    local_struct_1.segment_xe4 = 0x1020;
    process_struct_1020_808e(param_1);
    return;
}

pub fn process_struct_1020_4092(in_struct_1: *mut Struct660) -> *mut Struct660 {
    let local_struct_1: *mut Struct660;
    let local_struct_1_hi: *mut Struct660;

    zero_list_1008_3e38(in_struct_1);
    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    local_struct_1.field_0x6 = 0;
    local_struct_1.field_0x8 = 0;
    local_struct_1.field_0xa = 1;
    local_struct_1.field_0xc = 0;
    local_struct_1.field_0xe = 0;
    zero_list_1008_3e38((in_struct_1 & 0xffff0000 | ZEXT24(local_struct_1 + 1)));
    return in_struct_1;
}

pub unsafe fn process_struct_1020_2594(param_1: &mut Struct44) {
    let local_struct_1: *mut Struct376;
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    local_struct_1 = param_1;
    param_1.ptr_a_lo = s_fem36_wav_1050_270c;
    local_struct_1.ptr_a_hi = 0x1020;
    local_struct_1.offset_xe2 = (s_fem51_wav_1050_27a2 + 6);
    local_struct_1.segment_xe4 = 0x1020;
    process_struct_1020_808e(param_1);
    return;
}

pub fn pass1_1020_25c0(in_struct_1_1: *mut Struct649) {
    let pu_var1: *mut u16;

    let mut in_dx: u16;
    let local_struct_1: *mut Struct649;
    let local_struct_1_hi: *mut Struct649;
    let in_struct_1: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let fn_ptr_1: fn();

    in_struct_1 = CONCAT22(in_dx, in_ax);
    local_struct_1_hi = (in_struct_1_1 >> 0x10);
    local_struct_1 = in_struct_1_1;
    if (local_struct_1.field_0xee != 0) {
        fn_ptr_1 = (local_struct_1.field_0xee + 8);
        in_struct_1 = (**fn_ptr_1)();
    }
    if (local_struct_1.field_0xea == 0) {
        local_struct_1.field_0xea = 1;
        struct_ops1::process_struct_1000_179c(0x98, (in_struct_1 >> 0x10));
        if (in_struct_1 == 0) {
            local_6 = 0x0;
        } else {
            pu_var1 = &local_struct_1.field_0xcc;
            unsafe {
                *pu_var1 = *pu_var1 + 1;
            }
            local_6 = process_struct_1020_1738(
                in_struct_1,
                local_struct_1.field_0xcc,
                in_struct_1_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10,
            );
        }
        fn_ptr_1 = (*local_6 + 8);
        (**fn_ptr_1)(0x1000, local_6, (local_6 >> 0x10));
    }
    return;
}

// pass1_struct_1 *32 process_struct_1010_20ba (Struct372 *32 in_struct_372_ptr, char *32 in_string_1)
pub fn process_struct_1010_20ba(a: Address<Struct372>, b: u32) -> Struct2111 {
    let mut out: Struct2111 = Struct2111 {
        field_0x0: 0,
        field_0x1: 0,
        field_0x2: 0,
        field_0x3: 0,
        field_0x4: 0,
        field_0x5: 0,
        field_0x6: 0,
        field_0x7: 0,
        field_0x8: 0,
        field_0x9: 0,
        field_0xa: (),
        field_0xc: (),
        field_0xe: 0,
        field_0xf: 0,
        field_0x10: 0,
        field_0x11: 0,
        field_0x12: 0,
        field_0x13: 0,
        field_0x14: (),
        field_0x18: 0,
        field_0x19: 0,
        field_0x1a: 0,
        field_0x1b: 0,
        field_0x1c: 0,
        field_0x1d: 0,
        field_0x1e: 0,
        field_0x1f: 0,
        field_0x20: 0,
        field_0x21: 0,
        field_0x22: 0
    };
    out
}

pub unsafe fn process_struct_1020_808e(ctx: &mut AppContext, in_struct_1: &mut Struct44) {
    let pu_var1: *mut u16;
    let local_Struct591_ptr_2: *mut Struct376;
    let local_struct_1: *mut Struct376;
    let local_struct_1_hi: *mut Struct376;
    let mut local_6: u16;
    let mut local_4: u16;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    in_struct_1.ptr_a_lo = 0x82bc;
    local_struct_1.ptr_a_hi = 0x1020;
    local_struct_1.offset_xe2 = 0x8358;
    local_struct_1.segment_xe4 = 0x1020;
    if (in_struct_1 == 0x0) {
        pu_var1 = 0x0;
        local_Struct591_ptr_2 = 0x0;
    } else {
        pu_var1 = &local_struct_1.offset_xe2;
        local_Struct591_ptr_2 = local_struct_1_hi;
    }
    local_6 = CONCAT22(local_Struct591_ptr_2, pu_var1);
    *local_6 = ctx.s_1_1050_389a;
    pu_var1[1] = &ctx.PTR_LOOP_1050_1008;
    pass1_1008_57c4((in_struct_1 & 0xffff0000 | ZEXT24(&local_struct_1.struct_215_ptr_0xd2)));
    in_struct_1.ptr_a_lo = 0x380a;
    local_struct_1.ptr_a_hi = &ctx.PTR_LOOP_1050_1008;
    in_struct_1.ptr_a_lo = ctx.s_1_1050_389a;
    local_struct_1.ptr_a_hi = &ctx.PTR_LOOP_1050_1008;
    return;
}
