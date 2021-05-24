use crate::app_context::AppContext;
use crate::err_ops::{error_check_1000_17ce, set_error_mode_1010_8b14};
use crate::file_ops::misc::file_fn_1008_6414;
use crate::func_ptr_funcs::{call_fn_ptr_1000_5586, call_fn_ptr_1008_64a2};
use crate::list_funcs::zero_list_1008_6c90;
use crate::mem_funcs::alloc_mem::{alloc_mem_1000_0ed4, alloc_mem_1000_1708};
use crate::other_funcs::mixed_fn_1010_830a;
use crate::other_funcs::{big_fn_1010_b038, modify_list_1008_3f62};
use crate::pass::pass12_funcs::pass1_1008_c6fa;
use crate::pass::pass13_funcs::{pass1_1008_8faa, pass1_1008_ab12};
use crate::pass::pass14_funcs::{pass1_1008_3e54, pass1_1008_3e76, pass1_1008_3eb4, pass1_1008_431c, pass1_1008_4480, pass1_1008_5118, pass1_1008_5784, pass1_1008_5b12, pass1_1008_6562, pass1_1008_6604, pass1_1008_6cec, pass1_fn_1008_60e8};
use crate::pass::pass15_funcs::pass1_1020_bae6;
use crate::pass::pass16_funcs::pass1_1028_62c8;
use crate::pass::pass17_funcs::{pass1_1030_1cd8, pass1_1030_1d58, pass1_1030_2242, pass1_1030_25d8, pass1_1030_25f0, pass1_1030_2622, pass1_1030_266c, pass1_1030_5a52, pass1_1030_5b00, pass1_1030_627e, pass1_1030_62e4, pass1_1030_6522, pass1_1030_73a8, pass1_1030_7c28, pass1_1030_7eda, pass1_1030_7f1a, pass1_1030_8086, pass1_1030_8326, pass1_1030_8344, pass1_1030_835a};
use crate::pass::pass19_funcs::{pass1_1018_dd7c, pass1_1040_a5d0, pass1_1040_a626};
use crate::pass::pass2_funcs::{pass1_1010_e8f6, pass1_1010_eb66};
use crate::pass::pass3_funcs::{pass1_1028_45e2, pass1_1028_45fe};
use crate::pass::pass4_funcs::{pass1_1028_dc52, pass1_1028_e1bc, pass1_1028_e1ec, pass1_1028_e4ec};
use crate::pass::pass6_funcs::{pass1_1038_4d28, pass1_1038_4d3c, pass1_1038_4e78, pass1_1038_5050, pass1_1038_50e0};
use crate::pass::pass7_funcs::pass1_1018_17f0;
use crate::pass::pass8_funcs;
use crate::pass::pass_funcs::{pass1_1000_3d7a, pass1_1000_4906};
use crate::string_ops::misc::{copy_string_1000_3d3e, string_fn_1000_3f9c, string_fn_1008_64c8, wsprintf_func_1008_b69c, pass1_1028_933c, pass1_1030_532e, load_string_1010_847e, big_switch_statement_1020_c0d8, big_switch_statement_1020_c222, big_switch_statement_1020_bd80};
use crate::struct_ops::struct_ops_2::{process_struct_1000_179c, process_struct_1008_4772, process_struct_1008_574a, process_struct_1008_8e9e, process_struct_1010_1d48, process_struct_1010_9fee, process_struct_1040_a598, struct_fn_1000_160a};
use crate::structs::prog_structs_11::Struct706;
use crate::structs::prog_structs_12::Struct462;
use crate::structs::prog_structs_15::Struct507;
use crate::structs::prog_structs_16::{Struct457, Struct493};
use crate::structs::prog_structs_18::{Struct480, Struct504};
use crate::structs::prog_structs_1::Struct455;
use crate::structs::prog_structs_2::{Struct199, Struct318};
use crate::structs::prog_structs_20::{Struct503, Struct505, Struct511, Struct512};
use crate::structs::prog_structs_22::{Struct463, Struct467};
use crate::structs::prog_structs_23::Struct458;
use crate::structs::prog_structs_24::{Struct1176, Struct2111, Struct482, Struct489};
use crate::structs::prog_structs_25::{Struct402, Struct459};
use crate::structs::prog_structs_26::{Struct1114, Struct456, Struct481};
use crate::structs::prog_structs_27::{pass1_struct_2, Struct450, Struct451};
use crate::structs::prog_structs_28::Struct477;
use crate::structs::prog_structs_29::{Struct452, Struct478, Struct485};
use crate::structs::prog_structs_30::{Struct404, Struct453, Struct479, Struct483};
use crate::structs::prog_structs_31::{Struct187, Struct454, Struct466, Struct468, Struct484, Struct486, Struct487, Struct490, Struct498};
use crate::structs::prog_structs_4::Struct510;
use crate::structs::prog_structs_5::Struct469;
use crate::structs::prog_structs_7::{Struct376, Struct44};
use crate::sys_ops::win_msg::post_win_msg_1008_a0e4;
use crate::typedefs::HDC16;
use crate::ui_ops::msg_box::msg_box_1010_8bb4;
use crate::util::{CARRY2, CONCAT11, CONCAT12, CONCAT13, CONCAT22, CONCAT31, SBORROW2, SUB42, ZEXT24};
use crate::string_ops::load::load_string_1010_84e0;

pub unsafe fn pass1_1010_887a(ctx: &mut AppContext, param_1: u32, param_2: u32, param_3: u16) {
    let mut u_var1: u16;
    let mut u_var2: i32;
    let mut u_var3: u32;
    let mut u_var4: u32;
    let ctx.dx_reg: &mut  u16;
    let pu_var5: &mut  u16;
    let struct_a: &mut  Struct199;

    let mut i32_var6: i32;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let mut unaff_ss: u16;
    let u_var9: u8;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: [u8; 6];
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
    let mut local_4: u16;
    let temp_7ff107b7cdf: &mut  Struct450;

    local_4 = param_3 - 10;
  // u_var7 = (param_1  >> 0x10);
    pass8_funcs::pass1_1010_878c(param_1, (local_4 * 10 + 0x3382));
    if ((param_1 + 0x67c) != 0) {
        i32_var6 = local_4 * 10;
        pu_var5 = ctx.dx_reg;
        u_var1 = local_4;
        pass1_1008_6562(
            (param_1 + 0x67c),
            CONCAT22((i32_var6 + 0x3388), (i32_var6 + 0x338a)),
            (i32_var6 + 0x3386),
            local_4,
            ctx.dx_reg,
        );
        _local_8 = CONCAT22(pu_var5, u_var1);
      // u_var7 = (param_2  >> 0x10);
        local_c = *(param_2 + 0x14);
        _local_10 = process_struct_1008_4772(local_c);
        _local_14 = process_struct_1008_4772(_local_8);
      // struct_a = (_local_14  >> 0x10);
        u_var3 = (_local_14 + 4);
      // u_var8 = (_local_10  >> 0x10);
        i32_var6 = _local_10;
        if (u_var3 < (i32_var6 + 4)) {
            u_var3 = *(i32_var6 + 4);
        }
        u_var4 = (_local_14 + 8);
        if (u_var4 < (i32_var6 + 8)) {
            u_var4 = *(i32_var6 + 8);
        }
        u_var2 = u_var4;
        _local_18 = u_var4 & 0xffff | u_var3 << 0x10;
        u_var9 = -1;
        process_struct_1000_179c(0x1e, struct_a);
        if ((struct_a | u_var2) == 0) {
            u_var2 = 0;
            u_var8 = 0;
        } else {
            pass1_1008_6604(CONCAT22(struct_a, u_var2), _local_18, (_local_18 >> 0x10));
            u_var8 = ctx.dx_reg;
        }
        _local_1c = CONCAT22(u_var8, u_var2);
        pass1_1008_431c(CONCAT22(u_var8, u_var2), u_var9);
      // u_var8 = (_local_10  >> 0x10);
        local_1e = (local_16 - (_local_10 + 4)) / 2;
        local_20 = local_18 - (_local_10 + 8);
        pass1_1008_3e54(CONCAT22(unaff_ss, local_26), 0, local_20, local_1e);
        pass1_1008_4480(_local_1c, CONCAT22(unaff_ss, local_26), local_c);
        pass1_1008_3e76(CONCAT22(unaff_ss, local_26), 0, 0, 7);
        pass1_1008_4480(_local_1c, CONCAT22(unaff_ss, local_26), _local_8);
        (param_3 * 4 + param_2) = _local_1c;
    }
    return;
}

pub unsafe fn pass1_1010_89f0(
    param_1: &mut  Struct451,
    param_2: Vec<u8>,
    param_3: u8,
    param_2_00: u32,
) {
    let mut u_var1: i32;
    let u_var2: u8;
    let pcVar3: String;
    let extraout_var: u32;

    let ctx.dx_reg: &mut  Struct199;
    let struct_a: &mut  Struct199;
    let ctx.dx_reg: Vec<u8>;
    let local_DX_148: Vec<u8>;

    let local_SI_89: &mut  Struct452;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let local_8: String;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_41ff73ce2e1: String;

    pcVar3 = param_1.field_0x67c;
    u_var1 = &param_1.field_0x67e;
    _local_c = CONCAT22(u_var1, pcVar3);
    if ((u_var1 | pcVar3) != 0) {
        call_fn_ptr_1008_64a2(pcVar3, u_var1);
        u_var2 = error_check_1000_17ce(_local_c);
        pcVar3 = CONCAT31(extraout_var, u_var2);
    }
    set_error_mode_1010_8b14(param_1, param_2, (param_1.field_0xe82 * 4 + 0x24be));
    local_6 = CONCAT22(ctx.dx_reg, pcVar3);
    local_SI_89 = (param_1.field_0xe82 * 4);
    struct_a = ctx.dx_reg;
    if ((*(local_SI_89 + 0x24be) == pcVar3) && ((local_SI_89 + 0x24c0) == ctx.dx_reg)) {
        msg_box_1010_8bb4(param_1, param_2, pcVar3, ctx.dx_reg);
        struct_a = ctx.dx_reg;
    }
    process_struct_1000_179c(8, struct_a);
    if ((struct_a | pcVar3) == 0) {
        pcVar3 = 0x0;
        local_DX_148 = 0x0;
    } else {
        file_fn_1008_6414(CONCAT22(struct_a, pcVar3), local_6);
        local_DX_148 = ctx.dx_reg;
    }
    param_1.field_0x67c = pcVar3;
    *&param_1.field_0x67e = local_DX_148;
    param_1.field_0x680 = 0;
    if ((&param_1.field_0x67e | param_1.field_0x67c) != 0) {
        local_8 = (&ctx.PTR_LOOP_1050_0000 + 1);
        while (local_8 < 10) {
            temp_41ff73ce2e1 = (local_8 * 10);
            // WARNING: Load size is inaccurate
            pcVar3 = local_8;
            string_fn_1008_64c8(
                param_1.field_0x67c,
                CONCAT22((temp_41ff73ce2e1 + 0x2558), (temp_41ff73ce2e1 + 0x255a)),
                *(temp_41ff73ce2e1 + 0x2556),
                *(temp_41ff73ce2e1 + 0x2554),
            );
            _local_16 = CONCAT22(ctx.dx_reg, pcVar3);
            pass8_funcs::pass1_1010_86de(param_1, param_2, param_3, CONCAT22(ctx.dx_reg, pcVar3));
            (local_8 * 4 + param_2_00) = _local_16;
            local_8 = local_8 + 1;
        }
    }
    return;
}

pub unsafe fn pass1_1010_8c32(param_1: &mut  Struct453, param_2: u32) {
    let mut in_eax: u32;
    let mut u_var1: u16;
    let mut unaff_bp: u16;
    let ppVar2: &mut  Struct2111;
    let mut u_var3: u16;

  // u_var1 = (in_eax  >> 0x10);
    u_var3 = param_2;
    process_struct_1010_1d48(CONCAT22(u_var3, param_1), (param_2 >> 0x10));
    &param_1.field_0xa = 0;
    CONCAT22(u_var3, param_1) = 0x8ee2;
    param_1.field_0x2 = 0x1010;
    ppVar2 = pass8_funcs::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(unaff_bp, 3));
    param_1.field_0xa = ppVar2;
    param_1.field_0xc = (ppVar2 >> 0x10);
    return CONCAT22(u_var1, param_1);
}

pub unsafe fn pass1_1010_8c78(param_1: &mut  Struct376) {
    param_1.ptr_a_lo = 0x8ee2;
    (param_1 + 2) = 0x1010;
    pass8_funcs::pass1_1010_1d80(param_1);
    return;
}

pub unsafe fn pass1_1010_8ebc(param_1: u32, param_2: u8) {
    pass1_1010_8c78(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_8ef2(param_1: &mut  u16) {
    let mut u_var1: i32;
    let in_dx: &mut  Struct199;

    let local_bx_4: &mut  Struct454;
    let mut u_var2: u16;
    let ppVar3: &mut  Struct2111;
    let mut in_stack_0000fff6: u16;
    let mut local_8: u16;

  // u_var2 = (param_1  >> 0x10);
    local_bx_4 = param_1;
    unsafe {
        *param_1 = ctx.s_1_1050_389a;
    }
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    u_var1 = 0;
    &local_bx_4.field_0x4 = 0;
    &local_bx_4.field_0x8 = 0;
    unsafe {
        *param_1 = 0x9254;
    }
    local_bx_4.field_0x2 = 0x1010;
    process_struct_1000_179c(0x18, in_dx);
    if ((in_dx | u_var1) == 0) {
        &local_bx_4.field_0x4 = 0;
    } else {
        pass1_1030_1cd8(CONCAT22(in_dx, u_var1), 5, 5);
        local_bx_4.field_0x4 = u_var1;
        local_bx_4.field_0x6 = ctx.dx_reg;
    }
    ppVar3 = pass8_funcs::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(in_stack_0000fff6, 3));
    local_bx_4.field_0x8 = ppVar3;
    local_bx_4.field_0xa = (ppVar3 >> 0x10);
    return;
}

pub unsafe fn pass1_1010_8f78(param_1: &mut  Struct455) {
    let pu_var1: &mut  u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let local_bx_5: &mut  Struct455;
    let mut u_var4: u16;

  // u_var4 = (param_1  >> 0x10);
    local_bx_5 = param_1;
    param_1 = 0x9254;
    local_bx_5.field_0x2 = 0x1010;
    pu_var1 = local_bx_5.field_0x4;
    u_var2 = local_bx_5.field_0x6;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            ppc_var3 = *pu_var1;
            (**ppc_var3)();
        }
    }
    param_1 = ctx.s_1_1050_389a;
    local_bx_5.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub unsafe fn pass1_1010_8fba(param_1: &mut  Struct456) {
    let pp_var1: fn();

    let mut u_var2: u32;


    let local_bx_12: &mut  Struct456;
    let mut u_var3: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

  // u_var3 = (param_1  >> 0x10);
    local_bx_12 = param_1;
    pp_var1 = (local_bx_12.field_0x4 + 0x10);
    (**pp_var1)();
    _local_a = CONCAT22(ctx.dx_reg, in_ax);
    local_e = 0;
    loop {
        if (_local_a <= local_e) {
            return;
        }
        pp_var1 = (local_bx_12.field_0x4 + 4);
        u_var2 = _local_a;
        (**pp_var1)();
        if ((ctx.dx_reg | u_var2) != 0) {
            break;
        }
        local_e = local_e + 1;
    }
    pp_var1 = (local_bx_12.field_0x4 + 8);
    (**pp_var1)();
    return;
}

pub unsafe fn pass1_1010_9044(param_1: u32) {
    let pp_var1: fn();

    pp_var1 = ((param_1 + 4) + 0x10);
    (**pp_var1)();
    return;
}

pub unsafe fn pass1_1010_905e(param_1: &mut  Struct457, param_2: u32) {
    let pu_var1: &mut  u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let local_bx_4: &mut  Struct457;
    let mut u_var4: u16;

  // u_var4 = (param_1  >> 0x10);
    local_bx_4 = param_1;
    pu_var1 = local_bx_4.field_0x4;
    u_var2 = &local_bx_4.field_0x6;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            ppc_var3 = *pu_var1;
            (**ppc_var3)();
        }
    }
    &local_bx_4.field_0x4 = param_2;
    return;
}

pub unsafe fn pass1_1010_9092(param_1: &mut  Struct458) {
    let pp_var1: fn();
    let mut in_ax: i32;
    let mut u_var2: u32;
    let struct_a: &mut  Struct199;
    let paVar3: &mut  Struct199;

    let mut u_var4: u16;

    let local_bx_4: &mut  Struct458;
    let mut u_var5: u16;
    let mut u_var6: u32;
    let mut local_14: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

  // u_var5 = (param_1  >> 0x10);
    local_bx_4 = param_1;
    u_var6 = local_bx_4.field_0x4;
    pp_var1 = (local_bx_4.field_0x4 + 0x10);
    (**pp_var1)();
    local_6 = CONCAT22(struct_a, in_ax);
    paVar3 = struct_a;
    process_struct_1000_179c(0xc, struct_a);
    if ((paVar3 | in_ax) == 0) {
        in_ax = 0;
        u_var4 = 0;
    } else {
        pass1_1010_8ef2(CONCAT22(paVar3, in_ax));
        u_var4 = ctx.dx_reg;
    }
    local_e = 0;
    while (local_e < local_6) {
        pp_var1 = (local_bx_4.field_0x4 + 4);
        u_var2 = local_6;
        (**pp_var1)(0x1000, local_bx_4.field_0x4, local_e, u_var6);
        if ((ctx.dx_reg | u_var2) != 0) {
            pp_var1 = ((in_ax + 4) + 0xc);
            (**pp_var1)(0x1000, (in_ax + 4), u_var2, ctx.dx_reg);
        }
        local_e = local_e + 1;
    }
    return;
}

pub unsafe fn pass1_1010_9130(param_1: u32, param_2: u32, param_3: u16) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let mut u_var3: u32;
    let mut local_4: u16;

  // u_var2 = (param_1  >> 0x10);
    u_var3 = pass1_1030_1d58((param_1 + 4));
    if (u_var3 != 0) {
        u_var1 = (param_1 + 8);
        pass1_1010_c3c2(u_var1, (u_var1 >> 0x10), param_2, u_var3);
        return;
    }
    *param_2 = 0;
    return;
}

pub unsafe fn pass1_1010_9172(param_1: &mut  Struct459) {
    let pu_var1: &mut  u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let local_bx_4: &mut  Struct459;
    let mut u_var4: u16;
    Struct706 * *ppa_var5;
    let mut u_var6: u32;
    let mut local_8: u16;

  // u_var4 = (param_1  >> 0x10);
    local_bx_4 = param_1;
    pu_var1 = local_bx_4.field_0x4;
    u_var2 = local_bx_4.field_0x6;
    ppa_var5 = CONCAT22(u_var2, pu_var1);
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            ppc_var3 = *pu_var1;
            ppa_var5 = (**ppc_var3)();
        }
    }
    process_struct_1000_179c(0x18, (ppa_var5 >> 0x10));
    if (ppa_var5 == 0x0) {
        u_var6 = 0;
    } else {
        u_var6 = pass1_1030_1cd8(ppa_var5, 5, 5);
    }
    local_bx_4.field_0x4 = u_var6;
    local_bx_4.field_0x6 = (u_var6 >> 0x10);
    return;
}

pub unsafe fn pass1_1010_91cc(param_1: u32) {
    let pp_var1: fn();
    let mut u_var2: u16;
    let lVar3: u32;
    let mut local_4: u16;

  // u_var2 = (param_1  >> 0x10);
    pp_var1 = ((param_1 + 4) + 0x10);
    lVar3 = (**pp_var1)();
    if (lVar3 != 0) {
        pp_var1 = ((param_1 + 4) + 8);
        (**pp_var1)();
    }
    return;
}

pub unsafe fn pass1_1010_9210(param_1: u32) {
    let pp_var1: fn();
    let u_var2: u8;

    pp_var1 = ((param_1 + 4) + 0xc);
    u_var2 = (**pp_var1)();
    return u_var2;
}

pub unsafe fn pass1_1010_922e(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1010_8f78(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_9258(param_1: &mut  Struct402) -> &mut  Struct402 {
    pass8_funcs::pass1_1010_383a(param_1);
    param_1.field_0x0 = 0x958e;
    (param_1 + 2) = 0x1010;
    return param_1;
}

pub unsafe fn pass1_1010_927a(param_1: &mut  Struct404) {
    param_1.a = 0x958e;
    (param_1 + 2) = 0x1010;
    pass8_funcs::pass1_1010_3880(param_1);
    return;
}

pub unsafe fn pass1_1010_92e6(param_1: &mut  u16) {
    unsafe {
        *param_1 = 0x9566;
        (param_1 + 2) = 0x1010;
    }
    pass8_funcs::pass1_1010_2db2(param_1);
    return;
}

pub unsafe fn pass1_1010_9304(param_1: u16, param_2: u16, param_1_00: i32) {
    let mut in_ax: i32;
    let in_dx: &mut  Struct199;
    let mut local_4: u16;

    if (param_1_00 != 0) {
        process_struct_1000_179c(param_1_00 << 2, in_dx);
        return;
    }
    process_struct_1000_179c(0x1a, in_dx);
    if ((in_dx | in_ax) != 0) {
        pass1_1010_9258(in_ax, in_dx);
        return;
    }
    return;
}

pub unsafe fn pass1_1010_9372(
    param_1: &mut  Vec<u8>,
    param_2: Vec<u8>,
    param_3: Vec<u8>,
    param_4: Vec<u8>,
    param_5: Vec<u8>,
) {
    let u16_1: Vec<u8>;
    let mut char_2: u8;
    let local_struct_461_1: Vec<u8>;
    let local_SI__1: Vec<u8>;
    let u16_2: Vec<u8>;
    let mut bool_1: bool;
    let mut u32_1: u32;
    // fn_ptr_1: &mut  Vec<u8>;
    let mut temp_5ff2a2fc9a: u32;

    if (0 < param_4) {
        if (_g_bool_1050_3528 == 0x0) {
            fn_ptr_1 = (param_1 + 0x18);
            u16_1 = (**fn_ptr_1)();
            _g_bool_1050_3528 =
                pass8_funcs::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(local_SI__1, u16_1));
        }
        temp_5ff2a2fc9a = (param_1 + 0xc);
        u32_1 = pass8_funcs::pass1_1010_2e02(_g_bool_1050_3528, (temp_5ff2a2fc9a + 0x12));
        local_struct_461_1 = param_2 + 1;
        u16_2 = param_3 + (0xfffe < param_2);
        char_2 = (param_4 + -1) * 0x4;
        while (char_2 != '\0') {
            bool_1 = CARRY2(local_struct_461_1, local_struct_461_1);
            local_struct_461_1 = (local_struct_461_1 * 2);
            u16_2 = (u16_2 * 2 + bool_1);
            char_2 = char_2 + -1;
        }
        pass8_funcs::pass1_1010_2e30(
            _g_bool_1050_3528,
            local_struct_461_1 | u32_1,
            u16_2 | (u32_1 >> 0x10),
            param_5,
        );
    }
    return;
}

pub unsafe fn pass1_1010_93f0(param_1: &mut  Struct462) {
    let local_bx_4: &mut  Struct462;
    let mut u_var1: u16;
    let mut unaff_ss: u16;
    let mut u_var2: u32;
    let mut local_1c: [u8; 26];

  // u_var1 = (param_1  >> 0x10);
    local_bx_4 = param_1;
    if (&local_bx_4.field_0x56 == 0) {
        pass1_1010_9258(local_1c, unaff_ss);
        u_var2 = pass8_funcs::pass1_1010_398e(CONCAT22(unaff_ss, local_1c), 0, 0, 0);
        local_bx_4.field_0x56 = u_var2;
        local_bx_4.field_0x58 = (u_var2 >> 0x10);
        pass1_1010_927a(local_1c, unaff_ss);
    }
    return;
}

pub unsafe fn pass1_1010_9432(param_1: u32) {
    load_string_1010_847e(ctx.g_struct_73_1050_14cc, (param_1 + 0x16));
    return;
}

pub unsafe fn pass1_1010_944e(param_1: &mut  Struct463, param_2: u16, param_3: u16) {
    let mut u_var1: u32;
    let fn_ptr_1: fn();

    if (param_1.field_0x56 == 0) {
        fn_ptr_1 = (CONCAT22(param_2, param_1) + 0x10);
        (**fn_ptr_1)();
    }
    u_var1 = pass8_funcs::pass1_1010_2e02(CONCAT22(param_2, param_1), param_3);
    pass8_funcs::pass1_1010_2e5c(param_1, param_2, u_var1);
    return;
}

pub unsafe fn pass1_1010_9488(param_1: u16, param_2: u16, param_1_00: u32) -> bool {
    let mut iVar1: i32;
    let mut u_var2: u16;
    let ppVar3: &mut  Struct2111;
    let mut u_var4: u16;
    let mut in_stack_0000ffee: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let bool_1: bool;

    u_var4 = (param_1_00 + 0x12);
    ppVar3 = pass8_funcs::process_struct_1010_20ba(
        ctx.g_struct_var_1050_0ed0,
        CONCAT22((in_stack_0000ffee >> 0x10), 3),
    );
  // u_var2 = (ppVar3  >> 0x10);
    local_a = ppVar3;
    if (u_var4 == 0x32) {
        iVar1 = pass1_1010_a5ca(local_a, u_var2, 0x32);
        if (iVar1 != 0) {
            return 0;
        }
        u_var4 = 0x4d;
    } else {
        if (u_var4 == 0x3f) {
            iVar1 = pass1_1010_a5ca(local_a, u_var2, 0x3f);
            if (iVar1 != 0) {
                return 0;
            }
            u_var4 = 0x4e;
        }
    }
    iVar1 = pass1_1010_a5ca(local_a, u_var2, u_var4);
    bool_1 = (iVar1 == 0);
    return bool_1;
}

pub unsafe fn pass1_1010_9502(param_1: u32) {
    let mut temp_5fd6aa1926: u32;

    temp_5fd6aa1926 = (param_1 + 0x16);
    return (temp_5fd6aa1926 + 2);
}

pub unsafe fn pass1_1010_951a(param_1: u32, param_2: u8) {
    pass1_1010_927a(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_9540(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1010_92e6(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_95f8(in_struct_1: &mut  Struct376) {
    let pu_var1: &mut  u32;
    let mut u_var2: i32;
    let pu_var3: Vec<u8>;
    let local_struct_1: &mut  Struct376;
    let mut local_es_5: u16;
    let fn_ptr_1: fn();

  // local_es_5 = (in_struct_1  >> 0x10);
    local_struct_1 = in_struct_1;
    in_struct_1.ptr_a_lo = 0xa1c8;
    local_struct_1.ptr_a_hi = 0x1010;
    pu_var1 = local_struct_1.dc_handle_x0a;
    u_var2 = local_struct_1.u16_x0c;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            fn_ptr_1 = *pu_var1;
        }
        (**fn_ptr_1)();
    }
    pu_var1 = local_struct_1.u16_x0e;
    u_var2 = local_struct_1.u16_x10;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            fn_ptr_1 = *pu_var1;
        }
        (**fn_ptr_1)();
    }
    pu_var1 = local_struct_1.palette_handle_x12;
    pu_var3 = local_struct_1.u8_ptr_x14;
    if ((pu_var3 | pu_var1) != 0) {
        unsafe {
            fn_ptr_1 = *pu_var1;
        }
        (**fn_ptr_1)();
    }
    pass8_funcs::pass1_1010_1d80(in_struct_1);
    return;
}

pub unsafe fn pass1_1010_9674(param_1: u32) {
    let pu_var1: &mut  u32;
    let mut u_var2: i32;
    let local_bx_4: &mut  Struct466;
    let mut local_es_4: u16;
    let temp_862f6fe88cf: &mut  u32;
    let fn_ptr_1: fn();

  // local_es_4 = (param_1  >> 0x10);
    local_bx_4 = param_1;
    pu_var1 = local_bx_4.a;
    u_var2 = &local_bx_4.field_0x14;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            fn_ptr_1 = *pu_var1;
        }
        (**fn_ptr_1)();
    }
    &local_bx_4.a = 0;
    return;
}

pub unsafe fn pass1_1010_96a8(param_1: u32, param_2: u16) {
    let piVar1: &mut  i32;
    let mut local_es_6: u16;

  // local_es_6 = (param_1  >> 0x10);
    piVar1 = (param_1 + 0x1e);
    unsafe {
        *piVar1 = *piVar1 - param_2;
        if (*piVar1 < 0) {
            (param_1 + 0x1e) = 0;
        }
    }
    return;
}

pub unsafe fn pass1_1010_96c2(param_1: u32) {
    return (param_1 + 0x1e);
}

pub unsafe fn pass1_1010_96d0(in_struct_1: &mut  Struct467) {
    let piVar1: &mut  i32;
    let local_struct_1: &mut  Struct467;
    let mut local_es_4: u16;
    let mut local_DXAX_54: u32;
    let mut local_8: u16;

  // local_es_4 = (in_struct_1  >> 0x10);
    local_struct_1 = in_struct_1;
    if (local_struct_1.a != 0) {
        if (0 < local_struct_1.b) {
            piVar1 = &local_struct_1.b;
            unsafe {
                *piVar1 = *piVar1 + -1;
            }
        }
        if ((local_struct_1.b == 0) && (local_struct_1.c != 0)) {
            local_8 = 1;
            local_DXAX_54 = pass1_1030_8326();
            local_DXAX_54 = (local_DXAX_54 >> 0x10);
            if ((local_DXAX_54 != 0) || (0x32 < local_DXAX_54)) {
                local_8 = 5;
            }
            if ((local_DXAX_54 != 0) || (0x3c < local_DXAX_54)) {
                local_8 = 10;
            }
            if (local_struct_1.c < local_8) {
                local_8 = local_struct_1.c;
            }
            piVar1 = &local_struct_1.c;
            unsafe {
                *piVar1 = *piVar1 - local_8;
                if (*piVar1 < 0) {
                    local_struct_1.c = 0;
                }
            }
            if (0 < local_struct_1.c) {
                return local_8;
            }
            return 0xffff;
        }
    }
    return 0;
}

pub unsafe fn pass1_1010_9766(param_1: u32) {
    let mut iVar1: i32;
    let mut local_es_4: u16;

  // local_es_4 = (param_1  >> 0x10);
    (param_1 + 0x1a) = 1;
    pass1_1010_a0a0();
    iVar1 = pass1_1010_9f8c(param_1, 0x80);
    (param_1 + 0x1e) = iVar1 * 0x32;
    return;
}

pub unsafe fn pass1_1010_9794(in_struct_1: &mut  Struct468) {
    let mut iVar1: i32;
    let mut u_var2: u32;
    let pu_var3: Vec<u8>;
    let mut u_var4: u32;
    let pa_var5: &mut  Struct199;

    let mut u_var6: i32;

    let local_struct_1: &mut  Struct468;
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let mut u_var8: u32;
    let mut local_16: u32;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 4];
    let mut local_6: u32;
    let fn_ptr_1: fn();

  // u_var7 = (in_struct_1  >> 0x10);
    local_struct_1 = in_struct_1;
    if (local_struct_1.c == 0) {
        local_struct_1.c = 1;
        u_var4 = &local_struct_1.d;
        u_var6 = local_struct_1.b;
        pa_var5 = (u_var6 | u_var4);
        if (pa_var5 != 0x0) {
            fn_ptr_1 = u_var4;
            (**fn_ptr_1)();
            pa_var5 = ctx.dx_reg;
        }
        process_struct_1000_179c(0xc, pa_var5);
        u_var6 = pa_var5 | u_var4;
        if (u_var6 == 0) {
            pa_var5 = 0x0;
            u_var6 = 0;
        } else {
            pa_var5 = process_struct_1008_574a((u_var4 & 0xffff | ZEXT24(pa_var5) << 0x10));
        }
        local_struct_1.d = pa_var5;
        local_struct_1.b = u_var6;
        pass1_1008_5784(CONCAT22(unaff_ss, local_a), local_struct_1.a);
        loop {
            pu_var3 = local_a;
            pass1_1008_5b12(CONCAT22(unaff_ss, pu_var3));
            if ((ctx.dx_reg | pu_var3) == 0) {
                break;
            }
            iVar1 = (pu_var3 + 4);
            if ((iVar1 == 0x3e) || (iVar1 == 0x41)) {
                u_var8 = local_struct_1.a;
                (u_var8 + 10) = 0;
                u_var8 = local_struct_1.a;
                fn_ptr_1 = (local_struct_1.a + 0xc);
                (**fn_ptr_1)();
                u_var2 = local_struct_1.a;
                (u_var2 + 10) = 1;
                local_6 = 0;
                fn_ptr_1 = (&local_struct_1.d + 4);
                (**fn_ptr_1)(
                    &ctx.PTR_LOOP_1050_1008,
                    &local_struct_1.d,
                    CONCAT22(ctx.dx_reg, pu_var3),
                    u_var8,
                );
            }
        }
    }
    return;
}

pub unsafe fn pass1_1010_988c(param_1: &mut  Struct469, param_2: u16) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut i_var3: i32;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let local_bx_4: &mut  Struct469;
    let mut u_var6: u16;
    let mut unaff_ss: u16;
    let lVar7: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

  // u_var6 = (param_1  >> 0x10);
    local_bx_4 = param_1;
    pass1_1008_5784(CONCAT22(unaff_ss, local_a), local_bx_4.field_0xe);
    while {
        lVar7 = pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
      // u_var5 = (lVar7  >> 0x10);
        i_var3 = lVar7;
        if (lVar7 == 0) {
            return;
        }
        (i_var3 + 4) != param_2
    } {}
    i_var4 = (i_var3 + 6) + -1;
    (i_var3 + 6) = i_var4;
    if ((i_var4 < 1)
        && (
            pp_var1 = (local_bx_4.field_0xe + 0xc),
            (**pp_var1)(&ctx.PTR_LOOP_1050_1008, local_bx_4.field_0xe, lVar7),
            u_var2 = local_bx_4.field_0xe,
            (u_var2 + 8) == 0,
        ))
    {
        local_bx_4.field_0x16 = 1;
    }
    return;
}

pub unsafe fn pass1_1010_9f72(param_1: u32, param_2: u16) {
    let mut u_var1: u16;

  // u_var1 = (param_1  >> 0x10);
    pass1_1010_9fa6(param_1, u_var1, (param_1 + 0xe), param_2);
    return;
}

pub unsafe fn pass1_1010_9f8c(param_1: u32, param_2: u16) {
    let mut u_var1: u16;

  // u_var1 = (param_1  >> 0x10);
    pass1_1010_9fa6(param_1, u_var1, (param_1 + 10), param_2);
    return;
}

pub unsafe fn pass1_1010_9fa6(param_1: u16, param_2: u16, param_1_00: u32, param_2_00: i32) {
    let mut u_var1: u16;
    let mut unaff_ss: u16;
    let lVar2: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    if (param_1_00 != 0) {
        pass1_1008_5784(CONCAT22(unaff_ss, local_a), param_1_00);
        loop {
            lVar2 = pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
          // u_var1 = (lVar2  >> 0x10);
            if (lVar2 == 0) {
                break;
            }
            if ((lVar2 + 4) == param_2_00) {
                return (lVar2 + 6);
            }
        }
    }
    return 0;
}

pub unsafe fn pass1_1010_a0a0(param_1: u32) {
    let piVar1: &mut  i32;
    let mut i_var2: i32;
    let mut i_var3: i32;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let mut unaff_ss: u16;
    let mut bVar7: bool;
    let mut bVar8: bool;
    let lVar9: u32;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(unaff_ss, local_a), (param_1 + 10));
    local_c = 4;
    pass8_funcs::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(local_1c, 2));
    if ((u16_1050_13ae != 2) && (u16_1050_13ae != 1)) {
        local_c = 2;
    }
    loop {
        loop {
            lVar9 = pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
          // u_var6 = (lVar9  >> 0x10);
            i_var4 = lVar9;
            if (lVar9 == 0) {
                return;
            }
            i_var2 = (i_var4 + 4);
            if (i_var2 != 0x12) {
                break;
            }
            piVar1 = (i_var4 + 6);
            unsafe {
                bVar8 = SBORROW2(*piVar1, 2);
                i_var3 = *piVar1 + -2;
            }
            bVar7 = i_var3 == 0;
            // LAB_1010_a151:
            if (!bVar7 && bVar8 == (i_var3 < 0)) {
                // LAB_1010_a153:
                piVar1 = (i_var4 + 6);
                unsafe {
                    *piVar1 = *piVar1 - (i_var4 + 6) / local_c;
                }
            }
        }
        if (((i_var2 != 0x3e) && (i_var2 != 0x41)) && (i_var2 != 0x80)) {
            if (i_var2 == 0x83) {
                piVar1 = (i_var4 + 6);
                unsafe {
                    bVar8 = SBORROW2(*piVar1, 1);
                    i_var2 = *piVar1;
                }
                i_var3 = i_var2 + -1;
                bVar7 = i_var2 == 1;
                // goto LAB_1010_a151;
            }
            // goto LAB_1010_a153;
        }
        u_var5 = (i_var4 + 6) / 2;
        piVar1 = (i_var4 + 6);
        unsafe {
            *piVar1 = *piVar1 - u_var5;
        }
        if (u_var5 == 0) {
            u_var5 = 1;
        }
        process_struct_1010_9fee(param_1, u_var5, (i_var4 + 4));
    }
}

pub unsafe fn pass1_1010_a172(param_1: u32, param_2: u8) {
    pass1_1010_95f8(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_a198(param_1: &mut  u16, param_2: u8) {
    unsafe {
        *param_1 = ctx.s_1_1050_389a;
    }
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_a478(param_1: &mut  Struct376) {
    let pHVar1: &mut  HDC16;
    let mut u_var2: i32;
    let local_bx_4: &mut  Struct376;
    let mut u_var3: i32;
    let mut local_6: u16;
    let mut local_4: u16;

  // u_var3 = (param_1  >> 0x10);
    local_bx_4 = param_1;
    param_1.ptr_a_lo = 0xe9cc;
    local_bx_4.ptr_a_hi = 0x1010;
    local_bx_4.dc_handle_x0a = 0xe9dc;
    local_bx_4.u16_x0c = 0x1010;
    if (&local_bx_4.field_0x138 != 0) {
        if ((u_var3 | local_bx_4) == 0) {
            pHVar1 = 0x0;
            u_var2 = 0;
        } else {
            pHVar1 = &local_bx_4.dc_handle_x0a;
            u_var2 = u_var3;
        }
        pass8_funcs::pass1_1010_1ea6(*&local_bx_4.field_0x138, CONCAT22(u_var2, pHVar1));
    }
    &local_bx_4.field_0x138 = 0;
    if (param_1 == 0x0) {
        pHVar1 = 0x0;
        u_var3 = 0;
    } else {
        pHVar1 = &local_bx_4.dc_handle_x0a;
    }
    local_6 = CONCAT22(u_var3, pHVar1);
    *local_6 = ctx.s_1_1050_389a;
    pHVar1[1] = (HDC16) & ctx.PTR_LOOP_1050_1008;
    pass8_funcs::pass1_1010_1d80(param_1);
    return;
}

pub unsafe fn pass1_1010_a50c(param_1: u32, param_2: u32, param_3: u32) {
    let local_AX_16: &mut  Struct477;
    let local_AX_44: &mut  Struct478;
    let mut local_8: u16;
    let mut local_4: u16;

    local_AX_16 = param_1;
    local_AX_16 = local_AX_16 + 1;
    pass1_1000_4906((param_1 & 0xffff0000 | ZEXT24(local_AX_16)), 0, 0x94);
    local_AX_44 = (&local_AX_16.field_0x0 + (param_3 + 10) * 6);
    local_4 = local_AX_44.field_0x12;
    local_8 = local_AX_44.field_0xe;
    (*local_8)(
        0x1000,
        &local_AX_16.field_0x0 + local_4,
        param_1,
        param_2,
        param_3,
    );
    return;
}

pub unsafe fn pass1_1010_a568(param_1: u16, param_2: u16, param_3: u16) {
    let ppVar1: &mut  pass1_struct_2;
    let mut in_dx: u16;

    ppVar1 = pass1_1030_8344(
        ctx._g_bool_1050_5748,
        (ctx._g_bool_1050_5748 >> 0x10),
        0x8000001,
    );
    pass1_1030_2622(CONCAT22(in_dx, ppVar1), param_1_00);
    return;
}

pub unsafe fn pass1_1010_a58a(param_1: u16, param_2: u16, param_3: u16) {
    let mut local_DXAX_18: u32;

    local_DXAX_18._0_2_ = pass1_1030_8344(
        ctx._g_bool_1050_5748,
        (ctx._g_bool_1050_5748 >> 0x10),
        0x8000001,
    );
    pass1_1030_266c(local_DXAX_18, CONCAT22(param_1_00, local_DXAX_18));
    return;
}

pub unsafe fn pass1_1010_a5ac(param_1: u16, param_2: u16, param_1_00: u32) {
    let mut u_var1: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var1 = pass1_1030_73a8(param_1_00);
    return (u_var1 + 0x20);
}

pub unsafe fn pass1_1010_a5ca(param_1: u16, param_2: u16, param_3: u16) {
    let mut local_DXAX_18: u32;

    local_DXAX_18._0_2_ = pass1_1030_8344(
        ctx._g_bool_1050_5748,
        (ctx._g_bool_1050_5748 >> 0x10),
        0x8000001,
    );
    pass1_1030_2242(CONCAT22(local_DXAX_18, local_DXAX_18), param_1_00);
    return;
}

pub unsafe fn pass1_1010_a5ec(param_1: u16, param_2: u16, param_3: u16, param_4: u32) {
    let pp_var1: fn();
    let ppVar2: &mut  pass1_struct_2;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let mut in_dx: u16;
    let pu_var5: &mut  u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_4 != 0) {
        ppVar2 = pass1_1030_8344(
            ctx._g_bool_1050_5748,
            (ctx._g_bool_1050_5748 >> 0x10),
            0x8000001,
        );
        local_6 = CONCAT22(in_dx, ppVar2);
        pu_var5 = pass1_1030_73a8(param_4);
        u_var4 = (pu_var5 + 0x20);
        if (u_var4 != param_2_00) {
            u_var3 = param_2_00;
            pass1_1010_a5ca(param_1, param_2, u_var4);
            if ((u_var4 != 0x70) && (u_var3 < 0)) {
                pass1_1030_25d8(CONCAT22(in_dx, ppVar2), u_var3 + 1, u_var4);
            }
            unsafe {
                pp_var1 = (*pu_var5 + 8);
            }
            u_var4 = param_2_00;
            (**pp_var1)();
            if (param_2_00 != 0x70) {
                pass1_1010_a5ca(param_1, param_2, param_2_00);
                if (u_var4 < 0) {
                    pass1_1030_25d8(local_6, u_var4 - 1, param_2_00);
                }
            }
        }
    }
    return;
}

pub unsafe fn pass1_1010_a69c(param_1: u32, param_2: u16) {
    let struct_b: &mut  pass1_struct_2;
    let mut u_var1: u16;
    let mut in_dx: u16;
    let struct_c: &mut  Struct2111;
    let mut u_var2: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut bStack48: u8;
    let local_2f: u8;
    let mut local_2e: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_36 = ctx._g_bool_1050_5748;
  // local_34 = (ctx._g_bool_1050_5748  >> 0x10);
    struct_b = pass1_1030_8344(local_36, local_34, 0x8000001);
    if (param_2 == 1) {
        local_14 = 0;
        while (local_14 < 0x83) {
            u_var1 = pass1_1030_2242(CONCAT22(in_dx, struct_b), local_14);
            if (0x19 < u_var1) {
                local_16 = u_var1 - 5;
                if (local_16 < 0x19) {
                    local_16 = 0x19;
                }
                pass1_1030_25d8(CONCAT22(in_dx, struct_b), local_16, local_14);
            }
            local_14 = local_14 + 1;
        }
        // goto switchD_1010_aaef_caseD_b;
    }
    pass1_1030_25f0(CONCAT22(in_dx, struct_b), 0, param_2);
    struct_c = pass8_funcs::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(local_2e, 0x35));
    u_var2 = param_1;
  // local_36 = (param_1  >> 0x10);
    u_var1 = local_36;
    match (param_2) {
        10 | 0xc => local_36 = 0x1b,
        _ => {}
        // goto switchD_1010_aaef_caseD_b;
        0x10 => {
            pass8_funcs::pass1_1010_682e(struct_c, 1, 0x2d);
            if (struct_b.field_0x160 == 0) {}
            // goto switchD_1010_aaef_caseD_b;
            local_34 = 0x2d;
            // goto LAB_1010_a91f;
        }
        0x12 => {
            pass8_funcs::pass1_1010_682e(struct_c, 1, 0x16);
            pass8_funcs::pass1_1010_682e(struct_c, 1, 0x17);
            pass8_funcs::pass1_1010_682e(struct_c, 1, 0x18);
            pass8_funcs::pass1_1010_682e(struct_c, 1, 0x40);
            bStack48 = 0x3f;
            // goto LAB_1010_a96c;
        }
        0x13 => {
            local_34 = 0x35;
            // goto LAB_1010_a91f;
        }
        0x19 => {
            // goto switchD_1010_aaef_caseD_19;
        }
        0x1a => {
            bStack48 = 0xf;
            // goto LAB_1010_a96c;
        }
        0x1c => {
            bStack48 = 0x11;
            // goto LAB_1010_a96c;
        }
        0x1d | 0x24 => {
            pass1_1010_abd2(u_var2, local_36, 0x1e);
            local_34 = 0x5b;
            // goto LAB_1010_a91f;
        }
        0x1e => {
            struct_c = pass8_funcs::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, 0x1002b);
            pass8_funcs::pass1_1010_08c0(struct_c, 1, 2);
            struct_c =
                pass8_funcs::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(local_2e, 0x40));
            wsprintf_func_1008_b69c(struct_c);
            // goto switchD_1010_aaef_caseD_b;
        }
        0x22 => {
            bStack48 = 8;
            // goto LAB_1010_aabe;
        }
        0x23 => {
            bStack48 = 0xc;
            // goto LAB_1010_aabe;
        }
        0x25 => {
            pass1_1010_abd2(u_var2, local_36, 0x14);
            pass1_1010_abd2(u_var2, local_36, 0x1b);
            pass1_1010_abd2(u_var2, local_36, 0x1e);
            pass1_1010_abd2(u_var2, local_36, 0x22);
            pass1_1010_abd2(u_var2, local_36, 0x25);
            pass1_1010_abd2(u_var2, local_36, 0x28);
            pass1_1010_abd2(u_var2, local_36, 0x2a);
            pass1_1010_abd2(u_var2, local_36, 0x2d);
            pass1_1010_abd2(u_var2, local_36, 0x2f);
            pass1_1010_abd2(u_var2, local_36, 0x31);
            pass1_1010_abd2(u_var2, local_36, 0x35);
            pass1_1010_abd2(u_var2, local_36, 0x38);
            pass1_1010_abd2(u_var2, local_36, 0x3a);
            pass1_1010_abd2(u_var2, local_36, 0x3c);
            pass1_1010_abd2(u_var2, local_36, 0x48);
            pass1_1010_abd2(u_var2, local_36, 0x4f);
            pass1_1010_abd2(u_var2, local_36, 0x52);
            pass1_1010_abd2(u_var2, local_36, 0x54);
            pass1_1010_abd2(u_var2, local_36, 0x57);
            pass1_1010_abd2(u_var2, local_36, 0x5b);
            pass1_1010_abd2(u_var2, local_36, 0x5d);
            pass1_1010_abd2(u_var2, local_36, 0x62);
            pass1_1010_abd2(u_var2, local_36, 0x66);
            pass1_1010_abd2(u_var2, local_36, 0x68);
            pass1_1010_abd2(u_var2, local_36, 0x6c);
            // goto switchD_1010_aaef_caseD_19;
        }
        0x29 => local_36 = 0x25,
        0x2a => {
            bStack48 = 0xf;
            // goto LAB_1010_aabe;
        }
        0x2b => {
            bStack48 = 0x6e;
            // goto LAB_1010_a96c;
        }
        0x30 => local_36 = 0x54,
        0x33 => {
            pass1_1010_abd2(u_var2, local_36, 0x31);
            local_34 = 0x6c;
            // goto LAB_1010_a91f;
        }
        0x36 => {
            bStack48 = 0x13;
            // goto LAB_1010_aabe;
        }
        0x37 => {
            bStack48 = 0x2c;

            // LAB_1010_a96c:
            pass8_funcs::pass1_1010_682e(struct_c, 1, bStack48);
            // goto switchD_1010_aaef_caseD_b;
        }
        0x38 => {
            pass8_funcs::pass1_1010_682e(struct_c, 1, 0x28);
            if (struct_b.field_0x160 == 0) {}
            // goto switchD_1010_aaef_caseD_b;
            local_34 = 0x28;
        }
        // goto LAB_1010_a91f;
        0x39 => {
            bStack48 = 0x10;
            // goto LAB_1010_aabe;
        }
        0x3a => {
            bStack48 = 0x11;
            // goto LAB_1010_aabe;
        }
        0x3b => {
            bStack48 = 0x12;
            // LAB_1010_aabe:
            pass8_funcs::pass1_1010_6814(struct_c, 1, bStack48);
            // goto switchD_1010_aaef_caseD_b;
        }
        0x3c => {
            pass1_1010_abd2(u_var2, local_36, 0x14);
            local_34 = 0x62;
            // goto LAB_1010_a91f;
        }
        0x3d => {
            pass8_funcs::pass1_1010_682e(struct_c, 1, 0x66);
            if (struct_b.field_0x160 == 0) {}
            // goto switchD_1010_aaef_caseD_b;
            local_34 = 0x66;
            // LAB_1010_a91f:
            pass1_1010_abd2(u_var2, local_36, local_34);
            // goto switchD_1010_aaef_caseD_b;
        }
        0x3e => local_36 = 0x5d,
        0x3f => local_36 = 0x22,
        0x40 => local_36 = 0x57,
        0x41 => {
            local_36 = 0x4f;
        }
    }
    pass1_1010_abd2(u_var2, u_var1, local_36);
    // switchD_1010_aaef_caseD_b:
    bStack48 = param_2;
    local_2f = (param_2 >> 8);
    struct_c = pass8_funcs::process_struct_1010_20ba(
        ctx.g_struct_var_1050_0ed0,
        CONCAT13(local_2f, CONCAT12(bStack48, 0x37)),
    );
  // u_var1 = pass1_1008_ab12(struct_c, (struct_c  >> 0x10), bStack48);
    if (u_var1 != 0) {
        post_win_msg_1008_a0e4(struct_c, 0, 0, 1, 0, u_var1);
    }
    post_win_msg_1008_a0e4(struct_c, 0, 0, 1, 0, 0x3d);
    struct_c = pass8_funcs::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, 0x1002b);
    pass8_funcs::pass1_1010_043a(struct_c, 0x4000001, 6);
    return;
    // switchD_1010_aaef_caseD_19:
    (struct_c + 0x148) = 0x34;
    // goto switchD_1010_aaef_caseD_b;
}

pub unsafe fn pass1_1010_abd2(param_1: u16, param_2: u16, param_3: u16) {
    let pu_var1: &mut  u16;
    let mut u_var2: i32;
    let mut unaff_si: u16;
    let mut unaff_ss: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_6 = pass8_funcs::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(unaff_si, 0x35));
    u_var2 = local_6 + 10;
    _local_a = local_6 & 0xffff0000 | u_var2;
    local_c = 0;
    local_10 = param_1_00;
    _local_14 = CONCAT22(unaff_ss, &stack0x000a);
    loop {
        pu_var1 = _local_14;
        if (local_10 == 0) {
            return;
        }
        if (local_c != 0) {
            break;
        }
        if ((local_10 * 2 + u_var2) != 0) {
            local_c = 1;
            local_e = local_10;
        }
        _local_14 = (_local_14 & 0xffff0000 | (local_14 + 2));
        unsafe {
            local_10 = *pu_var1;
        }
    }
    pass8_funcs::pass1_1010_682e(local_6, 0, local_e);
    pass8_funcs::pass1_1010_682e(local_6, 1, local_10);
    return;
}

pub unsafe fn pass1_1010_ac62(param_1: u16, param_2: u16, param_1_00: i32) {
    let ppVar1: &mut  pass1_struct_2;
    let mut in_dx: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar1 = pass1_1030_8344(
        ctx._g_bool_1050_5748,
        (ctx._g_bool_1050_5748 >> 0x10),
        0x8000001,
    );
    return (&ppVar1.field_0x116 + param_1_00 * 2);
}

pub unsafe fn pass1_1010_ac92(param_1: u16, param_1_00: i32) {
    if (0 < param_1_00) && (param_1_00 < 0x43) {
        load_string_1010_847e(ctx.g_struct_73_1050_14cc, param_1_00 + 0x664);
        return;
    }
    return;
}

pub unsafe fn pass1_1010_acc0(param_1: u16, param_2: u16, param_1_00: u32) {
    let mut u_var1: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var1 = pass1_1030_73a8(param_1_00);
    if ((u_var1 + 0x12) != 4) {
        return 1;
    }
    return 0;
}

pub unsafe fn pass1_1010_acec(param_1: u32, param_2: u16) {
    if (param_2 == 1) {
        (param_1 + 0x12e) = 0;
    } else {
        if (param_2 != 5) {
            return;
        }
    }
    pass8_funcs::pass1_1010_1f62((param_1 & 0xffff0000 | (param_1 - 10)), param_2);
    return;
}

pub unsafe fn pass1_1010_ad22(param_1: u32) {
    let mut u_var1: u32;
    let pu_var2: Vec<u8>;

    let mut unaff_ss: u16;
    let mut local_8: u16;

    u_var1 = (param_1 + 0x138);
    pu_var2 = &stack0x0008;
    pass1_1030_627e(
        _PTR_LOOP_1050_5740,
        CONCAT22(unaff_ss, pu_var2),
        (u_var1 + 0x20),
    );
    if ((ctx.dx_reg | pu_var2) == 0) {
        return;
    }
    local_8 = ctx.dx_reg;
    pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, pu_var2, ctx.dx_reg);
    return;
}

pub unsafe fn pass1_1010_ad64(param_1: u16, param_2: u32, param_3: u32) {
    let mut u_var1: u32;
    let mut local_a: u32;

    if ((param_3 != 0) && (u_var1 = (param_3 + 0x2e), (u_var1 + 0x200) == 0x8000002)) {
        return;
    }
    pass1_1010_c58a(param_1, param_2, (param_2 >> 0x10), param_3);
    return;
}

pub unsafe fn pass1_1010_ada6(param_1: u16, param_2: u16, param_1_00: Vec<u8>, param_2_00: i32) {
    let mut u_var1: u16;
    let mut in_dx: u16;
    let mut local_6: u32;

    local_6 = 0;
    if (param_2_00 == 6) {
        if (param_1_00 == 0x0) {}
        // goto LAB_1010_adee;
        u_var1 = big_switch_statement_1020_c222(param_1_00);
    } else {
        if (param_2_00 != 7) {
            return 0;
        }
        if (param_1_00 == 0x0) {}
        // goto LAB_1010_adee;
        u_var1 = big_switch_statement_1020_c2f8(param_1_00);
    }
    local_6 = CONCAT22(in_dx, u_var1);
    // LAB_1010_adee:
    if (local_6 == 0) {
        local_6 = load_string_1010_847e(ctx.g_struct_73_1050_14cc, 0x531);
    }
    return local_6;
}

pub unsafe fn pass1_1010_ae12(param_1: u16, param_2: u16, param_1_00: u32, param_2_00: i32) {
    let mut u_var1: u16;
    let mut i_var2: i32;
    let mut in_dx: u16;
    let mut local_4: u16;

    if (param_2_00 == 6) {
        local_4 = 0;
        while (local_4 < 0x15) {
            u_var1 = big_switch_statement_1020_c222(local_4);
            i_var2 = pass1_1000_3d7a(param_1_00, u_var1, in_dx);
            if (i_var2 == 0) {
                return local_4;
            }
            local_4 = local_4 + 1;
        }
    } else {
        if (param_2_00 == 7) {
            local_4 = 0;
            while (local_4 < 0x11) {
                u_var1 = big_switch_statement_1020_c2f8(local_4);
                i_var2 = pass1_1000_3d7a(param_1_00, u_var1, in_dx);
                if (i_var2 == 0) {
                    return local_4;
                }
                local_4 = local_4 + 1;
            }
        }
    }
    return 0xffff;
}

pub unsafe fn pass1_1010_ae92(param_1: u32, param_2: u16, param_3: u16, param_4: u32) {
    let mut u_var1: u16;
    let mut u_var2: u32;
    let mut u_var3: u32;
    let ppVar4: &mut  Struct2111;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let u_var7: u8;
    let u_var8: u8;
    let u_var9: u8;
    let u_var10: u8;
    let mut u_var11: u16;
    let mut u_var12: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_8: u16;

    if (param_3 == 0x15) {
        u_var2 = pass1_1030_73a8(param_4);
        if (u_var2 != 0) {
            (u_var2 + 0x20) = param_2;
            return;
        }
    } else {
        if (param_3 < 0x16) {
            if (param_3 == '\x06') {
                pass1_1030_7f1a(param_4, param_2);
                u_var2 = pass1_1030_73a8(param_4);
              // u_var1 = pass1_1010_b028(param_1, (param_1  >> 0x10), u_var2);
                u_var3 = pass1_1030_8326();
                if (((u_var1 == 0xe) && ((u_var3 >> 0x10) != 0 || (0x32 < u_var3)))
                    && (param_2 == 1 || ((param_2 == 2 || (param_2 == 4)) || (param_2 == 3))))
                {
                    u_var12 = 0;
                    u_var1 = 0xb;
                    u_var9 = 1;
                    u_var10 = 0;
                    u_var11 = 0;
                    u_var6 = 0;
                    u_var7 = 0;
                    u_var8 = 0;
                    u_var5 = 0;
                    ppVar4 = pass8_funcs::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, 0x37);
                    post_win_msg_1008_a0e4(
                        ppVar4,
                        CONCAT22(u_var6, u_var5),
                        CONCAT11(u_var8, u_var7),
                        CONCAT11(u_var10, u_var9),
                        CONCAT22(u_var12, u_var11),
                        u_var1,
                    );
                    return;
                }
            } else {
                if (param_3 == 0x7) {
                    pass1_1030_7eda(param_4, param_2);
                    return;
                }
            }
        }
    }
    return;
}

pub unsafe fn pass1_1010_af66(param_1: u32) {
    let mut u_var1: u32;
    let paVar2: &mut  Struct493;
    let mut in_dx: i32;
    let mut in_stack_00000008: u16;
    let mut local_8: u16;
    let mut temp_5f97cf777f: u32;

    temp_5f97cf777f = (param_1 + 0x138);
    u_var1 = (temp_5f97cf777f + 0x24);
  // paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1  >> 0x10));
    if ((in_dx | paVar2) == 0) {
        return;
    }
    pass1_1038_5050(CONCAT22(in_dx, paVar2), in_stack_00000008);
    return;
}

pub unsafe fn pass1_1010_afa2(param_1: &mut  u32) {
    let mut u_var1: u32;
    let paVar2: &mut  Struct493;
    let mut in_dx: i32;
    let mut in_stack_00000008: u16;
    let mut local_8: u16;
    let mut temp_5fb906f77c: u32;

    temp_5fb906f77c = (param_1 + 0x138);
    u_var1 = (temp_5fb906f77c + 0x24);
  // paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1  >> 0x10));
    if ((in_dx | paVar2) == 0) {
        return;
    }
    pass1_1038_50e0(CONCAT22(in_dx, paVar2), in_stack_00000008);
    return;
}

pub unsafe fn pass1_1010_afde(param_1: u32, param_2: u16) {
    let mut u_var1: u32;
    let paVar2: &mut  Struct493;
    let pu_var3: Vec<u8>;
    let mut in_dx: i32;
    let mut u_var4: i32;
    let mut local_a: u16;
    let mut local_8: u16;

    u_var1 = (param_1 + 0x138);
    u_var1 = (u_var1 + 0x24);
  // paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1  >> 0x10));
    u_var4 = in_dx | paVar2;
    if (u_var4 == 0) {
        return;
    }
    pu_var3 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, param_2);
    pass1_1038_4e78(CONCAT22(in_dx, paVar2), pu_var3 & 0xffff | u_var4 << 0x10);
    return;
}

pub unsafe fn pass1_1010_b028(param_1: u16, param_2: u16, param_1_00: u32) {
    return (param_1_00 + 0xc);
}

pub unsafe fn pass1_1010_bf08(param_1: u16, param_2: u16, param_1_00: u32) {
    pass1_1028_e1bc();
    return;
}

pub unsafe fn pass1_1010_bf1e(param_1: u32, param_2: u32) {
    let mut u_var1: u32;
    let mut in_ax: i32;
    let mut in_i16_5: i32;
    let p_uvar2: &mut  u16;
    let struct_a: &mut  Struct199;
    let in_u16_6: &mut  Struct199;

    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u32;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_4: u16;

    pass1_1010_bf08(param_1, (param_1 >> 0x10), 0x1000000);
    local_4 = in_ax - 1;
    param_2 = local_4;
    in_i16_5 = local_4 * 0x18;
    in_u16_6 = struct_a;
    process_struct_1000_179c(in_i16_5, struct_a);
    _local_28 = CONCAT22(in_u16_6, in_i16_5);
    i_var3 = param_2;
  // u_var4 = (param_2  >> 0x10);
    if ((in_u16_6 | in_i16_5) == 0) {
        (i_var3 + 2) = 0;
    } else {
        call_fn_ptr_1000_5586(0x4092, 0x1020, local_4, 0x18, in_i16_5, in_u16_6);
        (i_var3 + 2) = _local_28;
    }
    pass1_1028_dc52(
        CONCAT22(unaff_ss, &local_16),
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        0x100,
    );
    local_1a = (i_var3 + 2);
    local_24 = 0;
    loop {
        pu_var2 = &local_16;
        pass1_1028_e4ec(CONCAT22(unaff_ss, pu_var2));
        if ((ctx.dx_reg | pu_var2) == 0) {
            break;
        }
        u_var1 = (pu_var2 + 8);
        if (u_var1 != 0) {
            modify_list_1008_3f62(local_1a, u_var1 & 0xffff0000 | (u_var1 + 4));
            (local_1a + 0xc) = local_24;
            local_24 = local_24 + 1;
            local_1a = local_1a & 0xffff0000 | (local_1a + 0x18);
        }
    }
    return;
}

pub unsafe fn pass1_1010_bffa(param_1: u32) {
    let pu_var1: &mut  u32;
    let pi_var2: &mut  i32;
    let mut u_var3: u32;
    let mut in_ax: i32;
    let mut u_var4: i32;
    let mut u_var5: u16;
    let puVar6: &mut  u16;
    let mut u_var7: i32;
    let in_dx: &mut  Struct199;
    let struct_a: &mut  Struct199;
    let struct_a_00: &mut  Struct199;


    let mut local_30: u16;
    let mut local_31: i32;
    let mut local_32: i32;
    let mut local_33: u16;
    let mut unaff_ss: u16;
    let mut local_2a: u32;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u32;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    process_struct_1000_179c(10, in_dx);
    local_6 = CONCAT22(in_dx, in_ax);
    pass1_1010_bf08(param_1, (param_1 >> 0x10), 0x2000000);
  // local_33 = (local_6  >> 0x10);
    local_31 = local_6;
    (local_31 + 8) = in_ax;
    if (in_ax == 0) {
        (local_31 + 8) = 1;
    }
    u_var4 = (local_31 + 8) << 2;
    struct_a_00 = struct_a;
    process_struct_1000_179c(u_var4, struct_a);
  // local_33 = (local_6  >> 0x10);
    local_31 = local_6;
    local_6 = u_var4;
    (local_31 + 2) = struct_a_00;
    if ((struct_a_00 | local_6) == 0) {
        (local_31 + 8) = 0;
    } else {
        u_var5 = (local_31 + 8) * 2;
        process_struct_1000_179c(u_var5, struct_a_00);
      // local_33 = (local_6  >> 0x10);
        local_31 = local_6;
        (local_31 + 4) = u_var5;
        (local_31 + 6) = struct_a_00;
        u_var4 = struct_a_00 | (local_31 + 4);
        if (u_var4 != 0) {
            mixed_fn_1010_830a(ctx.g_struct_73_1050_14cc, 0x1b4);
            pu_var1 = *local_6;
            unsafe {
                *pu_var1 = u_var4;
            }
            (pu_var1 + 2) = ctx.dx_reg;
            (local_6 + 4) = 0;
            pass1_1028_dc52(
                CONCAT22(unaff_ss, &local_18),
                (&ctx.PTR_LOOP_1050_0000 + 1),
                0,
                0x200,
            );
            local_1e = 1;
            loop {
                puVar6 = &local_18;
                pass1_1028_e4ec(CONCAT22(unaff_ss, puVar6));
                if ((ctx.dx_reg | puVar6) == 0) {
                    break;
                }
                pi_var2 = (puVar6 + 8);
                unsafe {
                    local_31 = *pi_var2;
                }
                local_22 = 0;
                u_var7 = local_31 - 1;
                u_var4 = u_var7;
                if (u_var7 < 10) {
                    u_var4 = pi_var2;
                    match (u_var7) {
                        0 => {
                            local_22 = 0x1b5;
                        }
                        1 => {
                            local_22 = 0x1b6;
                        }
                        2 => {
                            local_22 = 0x1b7;
                        }
                        3 => {
                            local_22 = 0x1b8;
                        }
                        4 => {
                            local_22 = 0x1b9;
                        }
                        5 => {
                            local_22 = 0x1ba;
                        }
                        6 => {
                            local_22 = 0x1bb;
                        }
                        7 => {
                            local_22 = 0x1bc;
                        }
                        8 => {
                            local_22 = 0x1bd;
                        }
                        9 => {
                            local_22 = 0x1be;
                        }
                    }
                }
                mixed_fn_1010_830a(ctx.g_struct_73_1050_14cc, local_22);
              // local_33 = (*local_6  >> 0x10);
                local_32 = *local_6;
                (local_32 + local_1e * 4) = u_var4;
                (local_32 + local_1e * 4 + 2) = ctx.dx_reg;
                u_var3 = (local_6 + 4);
                (u_var3 + local_1e * 2) = local_31;
                local_1e = local_1e + 1;
            }
            return;
        }
    }
    return;
}

pub unsafe fn pass1_1010_c1ba(param_1: u16, param_2: u16, param_3: u16) {
    let mut unaff_ss: u16;
    let lVar1: u32;
    let mut local_22: u16;
    let mut local_1c: u16;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_4: u16;

    local_4 = 0;
    pass1_1028_dc52(
        CONCAT22(unaff_ss, &local_16),
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        0x200,
    );
    local_1c = 1;
    while ((
        lVar1 = pass1_1028_e4ec(CONCAT22(unaff_ss, &local_16)),
        lVar1 != 0 && (local_1c != param_1_00),
    )) {
        local_1c = local_1c + 1;
    }
    return;
}

pub unsafe fn pass1_1010_c234(param_1: u32) {
    let mut u_var1: u32;
    let mut local_4: u16;

    u_var1 = pass1_1010_c28a(param_1);
    if (u_var1 == 0) {
        return 0;
    }
    u_var1 = pass1_1038_4d28(u_var1);
    return u_var1;
}

pub unsafe fn pass1_1010_c25e(param_1: u32, param_2: u32) {
    let paVar1: &mut  Struct1114;
    let mut local_4: u16;

    if (param_2 != 0) {
        paVar1 = pass1_1010_c28a(param_1);
        if (paVar1 != 0x0) {
            pass1_1038_4d3c(paVar1, param_2);
        }
    }
    return;
}

pub unsafe fn pass1_1010_c28a() {
    let mut u_var1: i32;
    let mut u_var2: u16;
    let paVar3: &mut  Struct493;
    let mut u_var4: i32;
    let ppVar5: &mut  Struct2111;
    let mut in_stack_0000ffec: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar5 = pass8_funcs::process_struct_1010_20ba(
        ctx.g_struct_var_1050_0ed0,
        CONCAT22((in_stack_0000ffec >> 0x10), 0x2f),
    );
  // u_var2 = (ppVar5  >> 0x10);
    u_var1 = (ppVar5 + 0x24);
    u_var4 = (ppVar5 + 0x26);
    if ((u_var4 | u_var1) != 0) {
        paVar3 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, u_var4);
        if ((u_var4 | paVar3) != 0) {
            return;
        }
    }
    return;
}

pub unsafe fn pass1_1010_c2d8(param_1: u16, param_2: u16, param_1_00: libc::c_long) {
    let mut u_var1: u16;
    let mut local_6: u32;

    if ((param_1_00 != 0)
        && (
          // u_var1 = (param_1_00  >> 0x10),
            local_6._0_2_ = *(param_1_00 + 0x2e),
            ((param_1_00 + 0x30) | local_6) != 0,
        ))
    {
        return;
    }
    return;
}

pub unsafe fn pass1_1010_c312() {
    return CONCAT22((ctx._PTR_LOOP_1050_65e2 + 2), *ctx._PTR_LOOP_1050_65e2);
}

pub unsafe fn pass1_1010_c320(param_1: u16, param_2: u16, param_1_00: u32, param_2_00: u32) {
    let in_dx: &mut  Struct199;
    let mut u_var1: u32;
    let mut in_resource_id: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: String = String::new();

    local_6 = param_1_00;
    if param_1_00 == 0 {
        process_struct_1000_179c(ctx, 0x100, in_dx);
        local_6 = param_1_00 & 0xffff | ZEXT24(in_dx) << 0x10;
    }
    u_var1 = pass1_1030_73a8(param_2_00);
    match u_var1 + 0x12 {
        1 | 2 | 4 => in_resource_id = 0x594,
        3 | 5 => in_resource_id = 0x593,
        6 => in_resource_id = 0x595,
        7 => in_resource_id = 0x596,
        8 => in_resource_id = 0x5f3,
        9 => in_resource_id = 0x664,
        _ => {
            *local_6 = 0;
            return;
        }
    }
    load_string_1010_84e0(
        ctx,
        0x3ff,
        local_6,
        in_resource_id,
    );
    return;
}

pub unsafe fn pass1_1010_c3c2(param_1: u16, param_2: u16, param_1_00: u32, param_2_00: u32) {
    let mut switch_var: u16;
    let in_dx: &mut  Struct199;
    let mut u_var1: u16;
    let mut unaff_ss: u16;
    let mut local_410: u16;
    let mut local_40e: u16;
    let mut local_40c: [u8; 1024];
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    local_6 = param_1_00;
    if (param_1_00 == 0) {
        process_struct_1000_179c(0x100, in_dx);
        local_6 = param_1_00 & 0xffff | ZEXT24(in_dx) << 0x10;
    }
    _local_a = pass1_1030_73a8(param_2_00);
  // u_var1 = (_local_a  >> 0x10);
    switch_var = (_local_a + 0xc);
    local_c = switch_var;
    big_switch_statement_1020_bd80(switch_var);
    copy_string_1000_3d3e(CONCAT22(unaff_ss, local_40c), CONCAT22(u_var1, switch_var));
    pass1_1030_8086(param_2_00);
    string_fn_1000_3f9c(
        local_6,
        (local_6 >> 0x10),
        s__s___lu_1050_38c5,
        &ctx.g_alloc_addr_1050_1050,
        local_40c,
    );
    return;
}

pub unsafe fn pass1_1010_c58a(param_1: u16, param_2: u16, param_3: u16, param_3_00: u32) {
    let ctx.ax_reg: &mut  Struct479;
    let mut u_var1: i32;
    let mut u_var2: i32;
    let in_dx: &mut  Struct199;

    let mut u_var3: i32;
    let struct_a: &mut  Struct199;
    let ctx.dx_reg: &mut  Struct199;
    let mut local_1a: u32;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    process_struct_1000_179c(0x18, in_dx);
    u_var1 = _local_AX__1;
    if ((in_dx | u_var1) == 0) {
        u_var1 = 0;
        u_var3 = 0;
    } else {
        process_struct_1040_a598((_local_AX__1 & 0xffff | ZEXT24(in_dx) << 0x10));
        u_var3 = ctx.dx_reg;
    }
    local_6 = CONCAT22(u_var3, u_var1);
    struct_a = (u_var3 | u_var1);
    if (struct_a == 0x0) {
        return;
    }
    local_e = 0;
    local_12 = 0;
    match (param_3) {
        5 => {
            local_e = &u16_1050_352c;
            local_12 = 0x300fa4
        }
        _ => {
            if (local_6 == 0x0) {
                return;
            }
            pass1_1040_a5d0(CONCAT22(u_var3, u_var1));
            error_check_1000_17ce(local_6);
            return;
        }
        10 => {
            local_e = &u16_1050_358c;
            local_12 = 0x510fb3
        }
        0xb => {
            local_e = &u16_1050_358c;
            local_12 = 0x510fb4
        }
        0xc => {
            local_e = &u16_1050_362e;
            local_12 = 0x300fb6
        }
        0x10 => {
            local_e = &PTR_DAT_1050_1805_1050_368e;
            local_12 = 0x3c0fc4
        }
        0x11 => {
            local_e = &PTR_DAT_1050_1805_1050_3706;
            local_12 = 0x4b0fc1
        }
        0x12 => {
            local_e = &u16_1050_379c;
            local_12 = 0x80fc5
        }
        0x13 => {
            pass1_1010_debe(
                CONCAT22(param_2, param_1),
                param_3,
                CONCAT22(u_var3, u_var1 + 0x10),
                CONCAT22(u_var3, u_var1 + 0xc),
                param_3_00,
            );
            local_e = &u16_1050_37ac;
            local_12 = 0x10fc6;
            struct_a = ctx.dx_reg
        }
        0x15 => {
            (u_var1 + 6) = param_3_00;
            (u_var1 + 10) = param_3
        }
        0x16 => {
            local_e = &u16_1050_37ae;
            local_12 = 0x40157
        }
        0x17 => {
            local_e = &u16_1050_37b6;
            local_12 = 0x2c0fd8;
        }
    }
    if (local_e != 0) {
        local_6.ptr_a_lo = local_12;
        u_var2 = local_12 * 10 + 2;
        process_struct_1000_179c(u_var2, struct_a);
        local_1a = CONCAT22(struct_a, u_var2);
        if ((struct_a | u_var2) == 0) {
            (u_var1 + 2) = 0;
        } else {
            local_1a = local_12;
            call_fn_ptr_1000_5586(
                0xa564,
                &ctx.PTR_LOOP_1050_1040,
                local_12,
                10,
                u_var2 + 2,
                struct_a,
            );
            (u_var1 + 2) = u_var2 + 2;
            (u_var1 + 4) = struct_a;
        }
        (u_var1 + 6) = param_3_00;
        (u_var1 + 10) = param_3;
        (u_var1 + 0x12) = local_12;
        pass1_1010_a50c(
            CONCAT22(param_2, param_1),
            local_e,
            CONCAT22(u_var3, u_var1),
        );
    }
    return;
}

pub unsafe fn pass1_1010_c864(param_1: u32, param_2: u32, param_3: u32) {
    let plVar1: &mut  long;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let u_var4: u8;
    let pu_var5: &mut  u32;
    let mut u_var6: i32;
    let pa_var7: &mut  Struct493;
    let mut u_var8: u16;
    let extraout_var: u32;


    let mut u_var10: i32;


    let mut i_var11: i32;
    let mut iVar12: i32;
    let mut u_var13: u16;
    let mut u_var14: u16;
    let mut uVar15: u16;
    let mut unaff_ss: u16;
    let mut uVar16: u16;
    let mut uVar17: u16;
    let mut local_1fe: u16;
    let mut local_1fc: u16;
    let mut local_1fa: u16;
    let mut local_1f8: u16;
    let mut local_1f6: u16;
    let mut local_1f0: u16;
    let mut local_1ee: u16;
    let mut local_19c: u32;
    let mut local_198: u16;
    let mut local_196: u16;
    let mut local_194: u16;
    let mut local_192: u16;
    let mut local_190: u16;
    let mut local_18e: u32;
    let mut local_18a: u16;
    let mut local_188: u16;
    let mut local_f6: u32;
    let mut local_62: u32;
    let mut local_5e: u16;
    let mut local_5c: u32;
    let mut local_58: u16;
    let mut local_56: u16;
    let mut local_54: [u8; 82];
    let mut u_var9: u32;

    local_56 = 0;
  // u_var13 = (param_3  >> 0x10);
    i_var11 = param_3;
    local_58 = param_3;
    u_var14 = 0;
    local_5c = 0;
    uVar16 = param_1;
  // uVar17 = (param_1  >> 0x10);
    pass1_1010_c320(uVar16, uVar17, 0, (i_var11 + 6));
    copy_string_1000_3d3e(CONCAT22(unaff_ss, local_54), CONCAT22(ctx.dx_reg, u_var14));
    local_62 = *(i_var11 + 2);
    (local_62 + 4) = param_2;
    pass1_1040_a626(local_62, CONCAT22(unaff_ss, local_54));
    local_5e = 0;
    local_192 = ctx.dx_reg;
    pass1_1000_4906(CONCAT22(unaff_ss, &local_f6), 0, 0x94);
    u_var4 = pass1_1000_4906(CONCAT22(unaff_ss, &local_18a), 0, 0x94);
    u_var9 = CONCAT31(extraout_var, u_var4);
    local_18e = 0;
    local_190 = 1;
    while (local_190 < 0x25) {
        pass1_1030_7c28((i_var11 + 6), local_190);
        u_var6 = u_var9;
        _local_194 = (u_var9 & 0xffff | local_192 << 0x10);
        local_192 = local_192 | u_var6;
        if (local_192 != 0) {
            big_switch_statement_1020_c0d8(local_190);
            _local_198 = CONCAT22(local_192, u_var6);
            u_var10 = local_192 | u_var6;
            if (u_var10 == 0) {
                copy_string_1000_3d3e((&local_f6)[local_18e], s_Null_Ptr_1050_38e1);
            } else {
                pass1_fn_1008_60e8(u_var6, local_192);
                (&local_f6 + local_18e) = u_var6;
                (&local_f6 + local_18e * 4 + 2) = u_var10;
            }
            u_var9 = _local_194 & 0xffff;
            (&local_18a)[local_18e * 2] = local_194;
            (&local_188)[local_18e * 2] = local_192;
            local_18e = local_18e + 1;
        }
        local_190 = local_190 + 1;
    }
    local_5c = local_18e;
    if (0x13 < local_18e) {
        local_5e = 1;
    }
    pu_var5 = &local_f6;
    pass1_1010_db2e(
        uVar16,
        uVar17,
        1,
        CONCAT22(unaff_ss, pu_var5),
        CONCAT22(unaff_ss, &local_18a),
        param_2,
        param_3,
    );
    local_56 = pu_var5;
    while (u_var9 = local_18e - 1, local_18e != 0) {
        local_18e._0_2_ = u_var9;
        local_19c = (&local_f6)[local_18e];
        _local_194 = local_19c;
        local_18e = u_var9;
        error_check_1000_17ce(local_19c);
    }
    uVar15 = 0x1000;
    local_18e = u_var9;
    pass1_1000_4906(CONCAT22(unaff_ss, &local_18a), 0, 0x54);
    u_var3 = (i_var11 + 6);
  // u_var14 = (u_var3  >> 0x10);
    iVar12 = u_var3;
    _local_194 = (iVar12 + 0x1e);
    u_var6 = (iVar12 + 0x20) | local_194;
    u_var9 = u_var6;
    if (u_var6 != 0) {
        local_18e = 0;
        loop {
            u_var3 = _local_194;
            ppc_var2 = (u_var3 + 0x10);
            ppc_var2(uVar15, _local_194, (_local_194 >> 0x10));
            if ((ctx.dx_reg < local_18e)
                || (ctx.dx_reg <= local_18e && (u_var9 <= local_18e)))
            {
                break;
            }
            ppc_var2 = (u_var3 + 4);
            ppc_var2(uVar15, _local_194, local_18e, local_18e);
            u_var6 = ctx.dx_reg | u_var9;
            if (u_var6 != 0) {
                uVar15 = SUB42(&PTR_LOOP_1050_1028, 0);
                pa_var7 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var9, ctx.dx_reg);
                _local_198 = CONCAT22(u_var6, pa_var7);
                if ((u_var6 | pa_var7) == 0) {
                    return;
                }
                iVar12 = &pa_var7.field_0xc;
                u_var9 = (iVar12 - 1);
                if (((0 < iVar12) && (!SBORROW2(iVar12, 1)))
                    && (u_var9 = (iVar12 - 6), iVar12 - 6 == 0 || (iVar12 - 1) < 5))
                {
                    plVar1 = (&local_18a + iVar12 * 2);
                    *plVar1 = *plVar1 + 1;
                }
            }
            local_18e = local_18e + 1;
        }
        u_var6 = ctx.dx_reg;
        pass1_1000_4906(CONCAT22(unaff_ss, &local_f6), 0, 0x54);
        pass1_1000_4906(CONCAT22(unaff_ss, &local_1f0), 0, 0x54);
        local_18e = 0;
        local_190 = 1;
        while (local_190 < 0x15) {
            if ((&local_18a + local_190 * 2) != 0) {
                u_var8 = big_switch_statement_1020_c222(local_190);
                u_var10 = u_var6 | u_var8;
                if (u_var10 == 0) {
                    copy_string_1000_3d3e((&local_f6)[local_18e], s_Null_Ptr_1050_38ea);
                } else {
                    pass1_fn_1008_60e8(u_var8, u_var6);
                    (&local_f6 + local_18e) = u_var8;
                    (&local_f6 + local_18e * 4 + 2) = u_var10;
                }
                u_var6 = (&local_188)[local_190 * 2];
                (&local_1f0)[local_18e * 2] = (&local_18a)[local_190 * 2];
                (&local_1ee)[local_18e * 2] = u_var6;
                local_18e = local_18e + 1;
            }
            local_190 = local_190 + 1;
        }
        if (local_5e == 0) {
          // iVar12 = (local_5c  >> 0x10) + CARRY2(local_5c, local_18e);
            local_5c = CONCAT22(iVar12, local_5c + local_18e);
            if ((iVar12 != 0) || (0x13 < local_5c + local_18e)) {
                local_5e = 1;
            }
        }
        if ((local_56 < local_58 - 2) && (_local_1f0 != 0)) {
            iVar12 = pass1_1010_dcac(uVar16, uVar17, local_56, param_2, param_3);
            pu_var5 = &local_f6;
            local_56 = iVar12 + 1;
            pass1_1010_db2e(
                uVar16,
                uVar17,
                local_56,
                CONCAT22(unaff_ss, pu_var5),
                CONCAT22(unaff_ss, &local_1f0),
                param_2,
                param_3,
            );
            local_56 = pu_var5;
        }
        while (u_var9 = local_18e - 1, local_18e != 0) {
            local_18e._0_2_ = u_var9;
            local_19c = (&local_f6)[local_18e];
            local_18e = u_var9;
            error_check_1000_17ce(local_19c);
        }
        if (local_5e != 0) {
            (i_var11 + 0x16) = 1;
        }
        local_18e = u_var9;
        pass1_1010_dc36(uVar16, uVar17, local_56, param_2, param_3);
    }
    return;
}

pub unsafe fn pass1_1010_cc56(param_1: u32, param_2: u32, param_3: u32) {
    let plVar1: &mut  long;
    let paVar2: &mut  Struct493;
    let pu_var3: &mut  u32;
    let mut u_var4: u16;
    let mut i_var5: i32;
    let lVar6: u32;
    let mut in_dx: i32;
    let mut u_var7: i32;

    let mut u_var8: i32;
    let mut u_var9: u16;
    let mut u_var10: u16;

    let mut local_1a0: u16;
    let mut local_19e: u16;
    let mut local_14c: u16;
    let mut local_14a: u16;
    let mut local_148: u16;
    let mut local_146: u16;
    let mut local_144: u16;
    let mut local_142: u16;
    let mut local_140: u16;
    let mut local_13e: u16;
    let mut local_13c: u16;
    let mut local_aa: u32;
    let mut local_16: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let mut temp_5f52790f98: u32;

  // u_var10 = (param_1  >> 0x10);
    u_var9 = param_1;
    temp_5f52790f98 = (u_var9 + 0x138);
    local_6 = (temp_5f52790f98 + 0x24);
  // paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_6, (local_6  >> 0x10));
    _local_a = CONCAT22(in_dx, paVar2);
    local_142 = in_dx | paVar2;
    if (local_142 != 0) {
        local_e = param_3;
        local_12 = 0;
        pass1_1000_4906(CONCAT22(ctx.stack_seg_reg, &local_aa), 0, 0x94);
        pass1_1000_4906(CONCAT22(ctx.stack_seg_reg, &local_13e), 0, 0x94);
        local_c = 0;
        local_10 = 0;
        local_16 = 0;
        local_140 = 1;
        while (local_140 < 0x25) {
            lVar6 = (_local_a + 0x26 + local_140 * 4);
            _local_144 = lVar6;
            if (lVar6 != 0) {
                big_switch_statement_1020_c0d8(local_140);
                u_var8 = lVar6;
                _local_14c = _local_14c & 0xffff | lVar6 << 0x10;
                u_var7 = local_142 | u_var8;
                local_148 = local_142;
                if (u_var7 == 0) {
                    copy_string_1000_3d3e((&local_aa)[local_16], s_Null_Ptr_1050_38f3);
                } else {
                    pass1_fn_1008_60e8();
                    (&local_aa + local_16) = u_var8;
                    (&local_aa + local_16 * 4 + 2) = u_var7;
                }
                (&local_13e)[local_16 * 2] = local_144;
                (&local_13c)[local_16 * 2] = local_142;
                local_16 = local_16 + 1;
            }
            local_140 = local_140 + 1;
        }
        local_10 = local_16;
        if (0x13 < local_16) {
            local_12 = 1;
        }
        pu_var3 = &local_aa;
        pass1_1010_db2e(
            u_var9,
            u_var10,
            1,
            CONCAT22(ctx.stack_seg_reg, pu_var3),
            CONCAT22(ctx.stack_seg_reg, &local_13e),
            param_2,
            param_3,
        );
        u_var8 = ctx.dx_reg;
        local_c = pu_var3;
        pass1_1000_4906(CONCAT22(ctx.stack_seg_reg, &local_13e), 0, 0x54);
        local_14c = 1;
        while (local_14c < 0x15) {
            local_146 = local_14c;
            if ((_local_a + 0x14e + local_14c * 4) != 0) {
                if (((0 < local_14c) && (!SBORROW2(local_14c, 1))) && ((local_14c - 1) < 6)) {
                    plVar1 = (&local_13e + local_14c * 2);
                    *plVar1 = *plVar1 + 1;
                }
            }
            local_14c = local_14c + 1;
        }
        pass1_1000_4906(CONCAT22(ctx.stack_seg_reg, &local_aa), 0, 0x54);
        pass1_1000_4906(CONCAT22(ctx.stack_seg_reg, &local_1a0), 0, 0x54);
        _local_14c = 0x10000;
        while (local_14a < 0x15) {
            if ((&local_13e + local_14a * 2) != 0) {
                u_var4 = big_switch_statement_1020_c222(local_14a);
                _local_144 = CONCAT22(u_var8, u_var4);
                u_var8 = u_var8 | u_var4;
                if (u_var8 == 0) {
                    copy_string_1000_3d3e((&local_aa)[local_14c], s_Null_Ptr_1050_38fc);
                } else {
                    pass1_fn_1008_60e8();
                    (&local_aa + local_14c) = u_var4;
                    (&local_aa + local_14c * 4 + 2) = u_var8;
                }
                u_var8 = (&local_13c)[local_14a * 2];
                (&local_1a0)[local_14c * 2] = (&local_13e)[local_14a * 2];
                (&local_19e)[local_14c * 2] = u_var8;
                _local_14c = _local_14c & 0xffff0000 | (local_14c + 1);
            }
            _local_14c = _local_14c & 0xffff | (local_14a + 1) << 0x10;
        }
        if (local_12 == 0) {
            local_10 = local_10 + local_14c;
            if (0x13 < local_10) {
                local_12 = 1;
            }
        }
        if ((local_c < local_e - 2) && (_local_1a0 != 0)) {
            i_var5 = pass1_1010_dcac(u_var9, u_var10, local_c, param_2, param_3);
            pu_var3 = &local_aa;
            local_c = i_var5 + 1;
            pass1_1010_db2e(
                u_var9,
                u_var10,
                local_c,
                CONCAT22(ctx.stack_seg_reg, pu_var3),
                CONCAT22(ctx.stack_seg_reg, &local_1a0),
                param_2,
                param_3,
            );
            local_c = pu_var3;
        }
        if (local_12 != 0) {
            (param_3 + 0x16) = 1;
        }
        pass1_1010_dc36(u_var9, u_var10, local_c, param_2, param_3);
    }
    return;
}

pub unsafe fn pass1_1010_cf36(param_1: u32, param_2: u32, param_3: u32) {
    let mut u_var1: u32;
    let mut u_var2: i32;
    let mut u_var3: i32;
    let pu_var4: &mut  u32;
    let mut u_var5: u32;
    let mut u_var6: u32;


    let mut u_var7: i32;
    let mut u_var8: i32;
    let mut u_var9: i32;
    let mut iVar10: i32;
    let mut i_var11: i32;
    let mut u_var12: u16;
    let mut u_var13: u16;
    let mut u_var14: u16;
    let mut unaff_ss: u16;
    let mut uVar15: u16;
    let mut uVar16: u16;
    let mut uVar17: u16;
    let mut local_156: u32;
    let mut local_152: u16;
    let mut local_150: u16;
    let mut local_14e: u16;
    let mut local_148: u16;
    let mut local_146: u16;
    let mut local_144: u16;
    let mut local_142: u16;
    let mut local_140: u32;
    let mut local_13c: u16;
    let mut local_13a: u32;
    let mut local_136: u16;
    let mut local_134: u16;
    let mut local_a2: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let mut local_4: u16;

    local_4 = 0;
    while {
      // u_var12 = (param_2  >> 0x10);
        iVar10 = param_2;
      // u_var13 = (param_3  >> 0x10);
        i_var11 = param_3;
        u_var1 = (i_var11 + 2);
        (local_4 * 10 + u_var1 + 4) = (local_4 * 2 + iVar10);
        local_4 = local_4 + 1;
        local_4 < 8
    } {}
    u_var5 = (i_var11 + 2);
    local_4 = 0;
    local_8 = u_var5;
    while {
        uVar15 = param_1;
      // uVar16 = (param_1  >> 0x10);
        big_fn_1010_b038(param_1, (i_var11 + 6));
        pass1_1040_a626(local_8, (u_var5 & 0xffff | ctx.dx_reg << 0x10));
        u_var7 = u_var5;
        local_4 = local_4 + 1;
        local_8 = local_8 & 0xffff0000 | (local_8 + 10);
        local_4 < 8
    } {}
    pass1_1010_dd5e(uVar15, uVar16, (i_var11 + 6));
    _local_c = CONCAT22(ctx.dx_reg, u_var7);
    u_var7 = ctx.dx_reg | u_var7;
    if (u_var7 != 0) {
        local_e = 0;
        pass1_1000_4906(CONCAT22(unaff_ss, &local_a2), 0, 0x94);
        pass1_1000_4906(CONCAT22(unaff_ss, &local_136), 0, 0x94);
        local_13a = 0;
        local_13c = 0;
        u_var1 = (i_var11 + 6);
        u_var5 = (u_var1 + 0x26);
        local_146 = 1;
        u_var6 = u_var5;
        while (u_var2 = u_var6, local_146 < 0x25) {
          // uVar17 = (u_var5  >> 0x10);
            if ((local_146 * 4 + _local_c) == 0) {
                u_var2 = u_var7;
                if (u_var5 != 0) {
                    u_var6 = pass1_1020_bae6(u_var5, CONCAT22(local_146, uVar17));
                    u_var9 = u_var6;
                    u_var2 = u_var7 | u_var9;
                    if (u_var2 != 0) {
                        if (local_e == 0) {
                            local_e = 1;
                        }
                        u_var3 = u_var9;
                        big_switch_statement_1020_c0d8(local_146);
                        u_var8 = u_var2 | u_var3;
                        if (u_var8 == 0) {
                            copy_string_1000_3d3e(
                                (&local_a2)[local_13a],
                                s_Null_Ptr_1050_390e,
                            );
                        } else {
                            pass1_fn_1008_60e8(u_var3, u_var2);
                            (&local_a2 + local_13a) = u_var3;
                            (&local_a2 + local_13a * 4 + 2) = u_var8;
                        }
                        (&local_136)[local_13a * 2] = u_var9;
                        (&local_134)[local_13a * 2] = u_var7;
                        local_13a = local_13a & 0xffff | (local_13a + 1) << 0x10;
                        // goto LAB_1010_d11d;
                    }
                }
            } else {
                if (local_e == 0) {
                    local_e = 1;
                }
                big_switch_statement_1020_c0d8(local_146);
                u_var9 = u_var7 | u_var2;
                if (u_var9 == 0) {
                    copy_string_1000_3d3e((&local_a2)[local_13a], s_Null_Ptr_1050_3905);
                } else {
                    pass1_fn_1008_60e8(u_var2, u_var7);
                    (&local_a2 + local_13a) = u_var2;
                    (&local_a2 + local_13a * 4 + 2) = u_var9;
                }
              // u_var14 = (_local_c  >> 0x10);
                u_var7 = (local_146 * 4 + _local_c + 2);
                (&local_136)[local_13a * 2] = (local_146 * 4 + _local_c);
                (&local_134)[local_13a * 2] = u_var7;
                local_13a = local_13a & 0xffff | (local_13a + 1) << 0x10;
                if (u_var5 == 0) {
                    u_var2 = 0;
                } else {
                    u_var6 = pass1_1020_bae6(u_var5, CONCAT22(local_146, uVar17));
                    u_var2 = u_var6;
                }
                u_var6 = u_var2;
                if (u_var2 == 0) {
                    local_13c = local_13c + 2;
                    u_var2 = u_var7;
                } else {
                    // LAB_1010_d11d:
                    (uVar15 + local_13a * 2 + 0xa4) = (iVar10 + local_13c * 2 + 0x10);
                    (uVar15 + (local_13a + 1) * 2 + 0xa4) = (iVar10 + (local_13c + 1) * 2 + 0x10);
                    local_13c = local_13c + 2;
                    u_var6 = (local_13a + 2);
                    local_13a = local_13a & 0xffff0000 | (local_13a + 2);
                    u_var2 = u_var7;
                }
            }
            local_146 = local_146 + 1;
            u_var7 = u_var2;
        }
        pu_var4 = &local_a2;
        pass1_1010_db2e(
            uVar15,
            uVar16,
            8,
            CONCAT22(unaff_ss, pu_var4),
            CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, &local_136)),
            param_2,
            param_3,
        );
        if (local_e != 0) {
            (i_var11 + 0x16) = 1;
        }
        while (
            local_13a = (local_13a - 1) << 0x10,
            local_13a != 0,
        ) {
            error_check_1000_17ce((&local_a2)[local_13a - 1]);
        }
        pass1_1010_dc36(uVar15, uVar16, pu_var4, param_2, param_3);
    }
    return;
}

pub unsafe fn pass1_1010_d24a(param_1: u32, param_2: u32, param_3: u32) {
    let mut u_var1: u32;
    let u_var2: u8;
    let mut u_var3: i32;
    let pu_var4: &mut  u32;
    let extraout_var: u32;



    let mut u_var6: i32;
    let mut u_var7: i32;
    let mut u_var8: i32;
    let mut i_var9: i32;
    let mut u_var10: u16;
    let mut unaff_ss: u16;
    let mut u_var11: u16;
    let mut u_var12: u16;
    let mut local_14e: u32;
    let mut local_14a: u16;
    let mut local_148: u16;
    let mut local_146: u16;
    let mut local_144: u16;
    let mut local_142: u16;
    let mut local_140: u16;
    let mut local_13e: u32;
    let mut local_13a: u16;
    let mut local_138: u16;
    let mut local_a6: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let mut local_4: u16;
    let mut u_var5: u32;

    local_4 = 0;
    while {
      // u_var10 = (param_3  >> 0x10);
        i_var9 = param_3;
        u_var1 = (i_var9 + 2);
        (local_4 * 10 + u_var1 + 4) = (local_4 * 2 + param_2);
        local_4 = local_4 + 1;
        local_4 < 8
    } {}
    u_var5 = (i_var9 + 2);
    local_4 = 0;
    local_8 = u_var5;
    while {
        u_var11 = param_1;
      // u_var12 = (param_1  >> 0x10);
        big_fn_1010_b038(param_1, (i_var9 + 6));
        pass1_1040_a626(local_8, (u_var5 & 0xffff | ctx.dx_reg << 0x10));
        u_var6 = u_var5;
        local_4 = local_4 + 1;
        local_8 = local_8 & 0xffff0000 | (local_8 + 10);
        local_4 < 8
    } {}
    pass1_1010_dd5e(u_var11, u_var12, (i_var9 + 6));
    if ((ctx.dx_reg | u_var6) != 0) {
        local_c = u_var6;
        local_a = ctx.dx_reg;
        pass1_1010_e8f6(u_var11, u_var12, (i_var9 + 6));
        _local_10 = CONCAT22(ctx.dx_reg, u_var6);
        local_12 = 0;
        u_var6 = ctx.dx_reg;
        pass1_1000_4906(CONCAT22(unaff_ss, &local_a6), 0, 0x94);
        u_var2 = pass1_1000_4906(CONCAT22(unaff_ss, &local_13a), 0, 0x94);
        u_var5 = CONCAT31(extraout_var, u_var2);
        local_13e = 0;
        local_140 = 1;
        while (local_140 < 0x25) {
            pass1_1030_7c28(_local_10, local_140);
            u_var3 = u_var5;
            u_var7 = u_var6 | u_var3;
            if (u_var7 != 0) {
                if (local_12 == 0) {
                    local_12 = 1;
                }
                big_switch_statement_1020_c0d8(local_140);
                u_var8 = u_var7 | u_var3;
                if (u_var8 == 0) {
                    copy_string_1000_3d3e((&local_a6)[local_13e], s_Null_Ptr_1050_3917);
                } else {
                    pass1_fn_1008_60e8(u_var3, u_var7);
                    (&local_a6 + local_13e) = u_var3;
                    (&local_a6 + local_13e * 4 + 2) = u_var8;
                }
                (&local_13a)[local_13e * 2] = (u_var5 & 0xffff);
                (&local_138)[local_13e * 2] = u_var6;
                local_13e = local_13e + 1;
                u_var5 = u_var5 & 0xffff;
                u_var7 = u_var6;
            }
            local_140 = local_140 + 1;
            u_var6 = u_var7;
        }
        pu_var4 = &local_a6;
        pass1_1010_db2e(
            u_var11,
            u_var12,
            8,
            CONCAT22(unaff_ss, pu_var4),
            CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, &local_13a)),
            param_2,
            param_3,
        );
        if (local_12 != 0) {
            (i_var9 + 0x16) = 1;
        }
        while (local_13e != 0) {
            local_13e._0_2_ = (local_13e - 1);
            error_check_1000_17ce((&local_a6)[local_13e]);
            local_13e = local_13e - 1;
        }
        pass1_1010_dc36(u_var11, u_var12, pu_var4, param_2, param_3);
    }
    return;
}

pub unsafe fn pass1_1010_d448(param_1: u32, param_2: u32, param_3: &mut  Struct481) {
    let pu_var1: &mut  u16;
    let pu_var2: Vec<u8>;
    let ctx.ax_reg: &mut  Struct480;
    let pu_var3: &mut  u16;
    let string_offset_b: String;
    let mut i_var4: i32;
    let mut u_var5: i32;
    let string_ptr_base_b: String;
    let struct_a_2: &mut  Struct481;
    let struct_a_1: &mut  Struct481;
    let string_ptr_base_a: String;
    let mut u_var6: u32;
    let mut u_var7: u16;
    let mut local_414: u16;
    let mut local_412: u32;
    let mut local_40e: u16;
    let mut local_40c: u16;
    let mut local_40a: u32;
    let mut local_406: u16;
    let mut local_404: u16;
    let mut string_offset_a: [u8; 1024];
    let temp_7ffb9c6ecdb: &mut  Struct1176;
    // temp_7e00f6c84af: &mut  Vec<u8>;

  // struct_a_1 = (param_3  >> 0x10);
    struct_a_2 = param_3;
    _local_406 = pass1_1030_73a8(struct_a_2.field_0x6);
  // u_var5 = (_local_406  >> 0x10);
    temp_7ffb9c6ecdb = _local_406;
    if ((u_var5 | temp_7ffb9c6ecdb) != 0) {
        local_40a = &temp_7ffb9c6ecdb.field_0x20;
        u_var5 = &temp_7ffb9c6ecdb.field_0x22;
        if ((u_var5 | local_40a) != 0) {
            local_40c = 0;
            pu_var3 = &local_40c;
          // u_var7 = (param_1  >> 0x10);
            pass1_1010_d984(
                param_1,
                u_var7,
                CONCAT22(string_ptr_base_a, pu_var3),
                3,
                local_40a & 0xffff | u_var5 << 0x10,
                &PTR_DAT_1050_1805_1050_368e,
                param_3,
            );
            string_offset_b = PTR_DAT_1050_1805_1050_368e;
            pu_var1 = struct_a_2.field_0x2;
            (pu_var1 + 4) = PTR_DAT_1050_1805_1050_368e;
            big_fn_1010_b038(param_1, struct_a_2.field_0x6);
            copy_string_1000_3d3e(
                CONCAT22(string_ptr_base_a, string_offset_a),
                CONCAT22(string_ptr_base_b, string_offset_b),
            );
            pass1_1040_a626(pu_var1, CONCAT22(string_ptr_base_a, string_offset_a));
            pu_var2 = struct_a_2.field_0x2;
            i_var4 = pu_var2;
            (i_var4 + 0xe) = PTR_LAB_1050_1821_1_1050_3690;
            string_fn_1000_3f9c(
                string_offset_a,
                string_ptr_base_a,
                s__u_1050_3920,
                &ctx.g_alloc_addr_1050_1050,
                local_40c,
            );
            pass1_1040_a626(
                (pu_var2 & 0xffff0000 | (i_var4 + 10)),
                CONCAT22(string_ptr_base_a, string_offset_a),
            );
            pu_var2 = struct_a_2.field_0x2;
            i_var4 = pu_var2;
            (i_var4 + 0x18) = PTR_LAB_1050_1823_1050_3692;
            u_var6 = pass1_1028_62c8(_local_406);
            string_fn_1000_3f9c(
                string_offset_a,
                string_ptr_base_a,
                s__u_1050_3923,
                &ctx.g_alloc_addr_1050_1050,
                u_var6,
            );
            pass1_1040_a626(
                (pu_var2 & 0xffff0000 | (i_var4 + 0x14)),
                CONCAT22(string_ptr_base_a, string_offset_a),
            );
            pass1_1010_dc36(param_1, u_var7, pu_var3, param_2, param_3);
        }
    }
    return;
}

pub unsafe fn pass1_1010_d5ae(param_1: u32, param_2: u32, param_3: &mut  Struct482) {
    let pu_var1: &mut  u16;
    let mut u_var2: u32;
    let pu_var3: &mut  u16;
    let local_AX_145: Vec<u8>;
    let local_AX_192: &mut  Struct484;
    let local_AX_261: &mut  Struct483;
    let mut u_var4: i32;

    let mut local_DX_145: u16;
    let local_bx_4: &mut  Struct482;
    let mut local_es_4: u16;

    let mut u_var5: u32;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut local_414: u16;
    let mut local_412: u32;
    let mut local_40e: u16;
    let mut local_40c: u16;
    let mut local_40a: u16;
    let mut local_408: u16;
    let mut local_406: u16;
    let mut local_404: u16;
    let mut local_402: [u8; 1024];
    let temp_5f9bbbd715: &mut  Struct483;
    let mut temp_5f3955c6bf: u32;

  // local_es_4 = (param_3  >> 0x10);
    local_bx_4 = param_3;
    _local_406 = pass1_1030_73a8(local_bx_4.field_0x6);
  // u_var4 = (_local_406  >> 0x10);
    local_40a = _local_406;
    if ((u_var4 | local_40a) != 0) {
        pass1_1028_45fe(local_40a, u_var4);
        if ((ctx.dx_reg | local_40a) != 0) {
            local_40c = 0;
            pu_var3 = &local_40c;
            u_var6 = param_1;
          // u_var7 = (param_1  >> 0x10);
            local_408 = ctx.dx_reg;
            pass1_1010_d984(
                u_var6,
                u_var7,
                CONCAT22(ctx.stack_seg_reg, pu_var3),
                3,
                CONCAT22(ctx.dx_reg, local_40a),
                &PTR_DAT_1050_1805_1050_3706,
                param_3,
            );
            local_AX_145 = PTR_DAT_1050_1805_1050_3706;
            pu_var1 = &local_bx_4.field_0x2;
            (pu_var1 + 4) = PTR_DAT_1050_1805_1050_3706;
            big_fn_1010_b038(u_var6, (param_1 >> 0x10), local_bx_4.field_0x6);
            copy_string_1000_3d3e(
                CONCAT22(ctx.stack_seg_reg, local_402),
                CONCAT22(local_DX_145, local_AX_145),
            );
            pass1_1040_a626(pu_var1, CONCAT22(ctx.stack_seg_reg, local_402));
            u_var2 = &local_bx_4.field_0x2;
            local_AX_192 = u_var2;
            local_AX_192 = &local_AX_192.field_0xa;
            local_AX_192.field_0xe = PTR_LAB_1050_1821_1_1050_3708;
            string_fn_1000_3f9c(
                local_402,
                ctx.stack_seg_reg,
                s__u_1050_3926,
                &ctx.g_alloc_addr_1050_1050,
                local_40c,
            );
            pass1_1040_a626(
                (u_var2 & 0xffff0000 | ZEXT24(local_AX_192)),
                CONCAT22(ctx.stack_seg_reg, local_402),
            );
            temp_5f9bbbd715 = &local_bx_4.field_0x2;
            local_AX_261 = temp_5f9bbbd715;
            local_AX_261 = &local_AX_261.field_0x14;
            local_AX_261.field_0x18 = PTR_LAB_1050_1823_1050_370a;
            u_var5 = pass1_1028_45e2(_local_406);
            string_fn_1000_3f9c(
                local_402,
                ctx.stack_seg_reg,
                s__u_1050_3929,
                &ctx.g_alloc_addr_1050_1050,
                u_var5,
            );
            pass1_1040_a626(
                (temp_5f9bbbd715 & 0xffff0000 | ZEXT24(local_AX_261)),
                CONCAT22(ctx.stack_seg_reg, local_402),
            );
            pass1_1010_dc36(u_var6, u_var7, pu_var3, param_2, param_3);
        }
    }
    return;
}

pub unsafe fn pass1_1010_d710(param_1: u32, param_2: Vec<u8>, param_3: &mut  Struct485) {
    let mut u_var1: u32;
    let mut u_var2: i32;
    let pu_var3: &mut  u32;
    let mut u_var4: u32;
    let mut u_var5: u32;


    let mut u_var6: i32;
    let mut u_var7: i32;
    let local_SI_15: Vec<u8>;
    let struct_a_2: &mut  Struct485;
    let local_es_15: Vec<u8>;
    let struct_a_1: &mut  Struct485;
    let mut u_var8: u16;

    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut local_152: u32;
    let mut local_14e: u16;
    let mut local_14c: u16;
    let mut local_14a: u16;
    let mut local_144: u16;
    let mut local_142: u16;
    let mut local_140: u32;
    let mut local_13c: u16;
    let mut local_13a: u32;
    let mut local_136: u16;
    let mut local_134: u16;
    let mut local_a2: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let local_4: Vec<u8>;
    let mut temp_5f71e254c5: u32;
    let mut temp_5fa9a51261: u32;

    local_4 = 0x0;
    while {
      // local_es_15 = (param_2  >> 0x10);
        local_SI_15 = param_2;
      // struct_a_1 = (param_3  >> 0x10);
        struct_a_2 = param_3;
        temp_5fa9a51261 = struct_a_2.field_0x2;
        (local_4 * 10 + temp_5fa9a51261 + 4) = (local_SI_15 + local_4 * 2);
        local_4 = local_4 + 1;
        local_4 < 4
    } {}
    u_var4 = struct_a_2.field_0x2;
    local_4 = 0x0;
    local_8 = u_var4;
    while {
        u_var9 = param_1;
      // u_var10 = (param_1  >> 0x10);
        big_fn_1010_b038(param_1, struct_a_2.field_0x6);
        pass1_1040_a626(local_8, (u_var4 & 0xffff | ctx.dx_reg << 0x10));
        u_var6 = u_var4;
        local_4 = local_4 + 1;
        local_8 = local_8 & 0xffff0000 | (local_8 + 10);
        local_4 < 4
    } {}
    pass1_1010_dd5e(u_var9, u_var10, struct_a_2.field_0x6);
    _local_c = CONCAT22(ctx.dx_reg, u_var6);
    u_var6 = ctx.dx_reg | u_var6;
    if (u_var6 != 0) {
        local_e = 0;
        pass1_1000_4906(CONCAT22(ctx.stack_seg_reg, &local_a2), 0, 0x94);
        pass1_1000_4906(CONCAT22(ctx.stack_seg_reg, &local_136), 0, 0x94);
        local_13a = 0;
        local_13c = 0;
        temp_5f71e254c5 = struct_a_2.field_0x6;
        u_var4 = (temp_5f71e254c5 + 0x26);
        local_142 = 1;
        u_var5 = u_var4;
        while (u_var2 = u_var5, local_142 < 0x25) {
            if ((local_142 * 4 + _local_c) != 0) {
                if (local_e == 0) {
                    local_e = 1;
                }
                big_switch_statement_1020_c0d8(local_142);
                u_var7 = u_var6 | u_var2;
                if (u_var7 == 0) {
                    copy_string_1000_3d3e((&local_a2)[local_13a], s_Null_Ptr_1050_392c);
                } else {
                    pass1_fn_1008_60e8(u_var2, u_var6);
                    (&local_a2 + local_13a) = u_var2;
                    (&local_a2 + local_13a * 4 + 2) = u_var7;
                }
              // u_var8 = (_local_c  >> 0x10);
                u_var6 = (local_142 * 4 + _local_c + 2);
                (&local_136)[local_13a * 2] = (local_142 * 4 + _local_c);
                (&local_134)[local_13a * 2] = u_var6;
                u_var1 = local_13a & 0xffff;
                local_13a = u_var1 | (local_13a + 1) << 0x10;
                if (u_var4 == 0) {
                    u_var2 = 0;
                } else {
                  // u_var5 = pass1_1020_bae6(u_var4, CONCAT22(local_142, (u_var4  >> 0x10)));
                    u_var2 = u_var5;
                }
                u_var5 = u_var2;
                if (u_var2 == 0) {
                    local_13c = local_13c + 2;
                } else {
                    local_13a._0_2_ = u_var1;
                    (u_var9 + local_13a * 2 + 0xa4) = (local_SI_15 + local_13c * 2 + 8);
                    (u_var9 + (local_13a + 1) * 2 + 0xa4) = (local_SI_15 + (local_13c + 1) * 2 + 8);
                    local_13c = local_13c + 2;
                    u_var5 = (local_13a + 2);
                    local_13a = CONCAT22(local_13a + 1, local_13a + 2);
                }
            }
            local_142 = local_142 + 1;
        }
        pu_var3 = &local_a2;
        pass1_1010_db2e(
            u_var9,
            u_var10,
            4,
            CONCAT22(ctx.stack_seg_reg, pu_var3),
            CONCAT13((ctx.stack_seg_reg >> 8), CONCAT12(ctx.stack_seg_reg, &local_136)),
            param_2,
            param_3,
        );
        if (local_e != 0) {
            struct_a_2.field_0x16 = 1;
        }
        while (
            local_13a = (local_13a - 1) << 0x10,
            local_13a != 0,
        ) {
            error_check_1000_17ce((&local_a2)[local_13a - 1]);
        }
        pass1_1010_dc36(u_var9, u_var10, pu_var3, param_2, param_3);
    }
    return;
}

pub unsafe fn pass1_1010_d984(
    param_1: u16,
    param_2: u16,
    param_1_00: &mut  i32,
    param_4: u16,
    param_5: u32,
    param_6: u32,
    param_7: u32,
) {
    let mut u_var1: u16;
    let local_AX_30: Vec<u8>;
    let mut u_var2: u16;
    let mut i_var3: i32;

    let mut u_var4: i32;
    let local_bx_142: &mut  Struct486;
    let mut i_var5: i32;
    let local_es_142: Vec<u8>;
    let local_es_179: Vec<u8>;
    let string_c: String;
    let mut local_41c: u16;
    let mut local_41a: u16;
    let mut string_a: [u8; 1024];
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut string_b: [u8; 8];
    let mut local_4: u16;
    let temp_5fddd85d53: Vec<u8>;
    let temp_5f0cc5ef4a: Vec<u8>;

    local_4 = param_2_00;
    pass1_1008_5784(CONCAT22(string_c, string_b), param_5);
    loop {
        local_AX_30 = string_b;
        pass1_1008_5b12(CONCAT22(string_c, local_AX_30));
        _local_10 = CONCAT22(ctx.dx_reg, local_AX_30);
        u_var4 = ctx.dx_reg | local_AX_30;
        if (u_var4 == 0) {
            return;
        }
        local_12 = (local_AX_30 + 10);
        u_var2 = 0;
        local_16 = 0;
        if ((local_AX_30 + 4) == 0) {
            if ((local_AX_30 + 6) == 0) {
                if ((local_AX_30 + 8) == 0) {
                    return;
                }
                u_var2 = big_switch_statement_1020_c2f8(*(local_AX_30 + 8));
            } else {
                u_var2 = big_switch_statement_1020_c222((local_AX_30 + 6));
            }
        } else {
            big_switch_statement_1020_c0d8((local_AX_30 + 4));
        }
        local_16 = CONCAT22(u_var4, u_var2);
        local_18 = (_local_10 + 0xc);
        *param_1_00 = *param_1_00 + local_18;
      // local_es_142 = (param_7  >> 0x10);
        local_bx_142 = param_7;
        temp_5fddd85d53 = local_bx_142.field_0x4;
        i_var3 = local_bx_142.field_0x2 + local_4 * 10;
        _local_41c = CONCAT22(temp_5fddd85d53, i_var3);
      // local_es_179 = (param_6  >> 0x10);
        i_var5 = param_6;
        (i_var3 + 4) = (local_4 * 2 + i_var5);
        string_fn_1000_3f9c(
            string_a,
            string_c,
            s__u_1050_3935,
            &ctx.g_alloc_addr_1050_1050,
            local_12,
        );
        pass1_1040_a626(_local_41c, CONCAT22(string_c, string_a));
        temp_5f0cc5ef4a = local_bx_142.field_0x4;
        i_var3 = local_bx_142.field_0x2 + (local_4 + 1) * 10;
        _local_41c = CONCAT22(temp_5f0cc5ef4a, i_var3);
        (i_var3 + 4) = ((local_4 + 1) * 2 + i_var5);
        copy_string_1000_3d3e(CONCAT22(string_c, string_a), local_16);
        pass1_1040_a626(_local_41c, CONCAT22(string_c, string_a));
        i_var3 = (local_4 + 2) * 10 + local_bx_142.field_0x2;
        u_var1 = local_bx_142.field_0x4;
        _local_41c = CONCAT22(u_var1, i_var3);
        (i_var3 + 4) = ((local_4 + 2) * 2 + i_var5);
        local_4 = local_4 + 3;
        string_fn_1000_3f9c(
            string_a,
            string_c,
            s__u_1050_3938,
            &ctx.g_alloc_addr_1050_1050,
            local_18,
        );
        pass1_1040_a626(_local_41c, CONCAT22(string_c, string_a));
    }
}

pub unsafe fn pass1_1010_db2e(
    param_1: u16,
    param_2: u16,
    param_1_00: &mut  Struct487,
    param_2_00: u32,
    param_5: u32,
    param_6: u32,
    param_7: &mut  i32,
) -> &mut  Struct187 {
    let mut u_var1: u16;
    let mut i_var2: i32;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let unaff_ss: String;
    let local_5e: &mut  Struct487;
    let local_5c: &mut  Struct489;
    let local_5a: &mut  Struct487;
    let mut local_58: u16;
    let mut local_56: u16;
    let mut local_54: [u8; 82];

    local_5e = param_1_00;
    local_5a = param_1_00;
    local_5c = 0x0;
    loop {
      // u_var6 = (param_7  >> 0x10);
        i_var3 = param_7;
        if ((*param_7 - 1) < local_5e) {
            return local_5e;
        }
        u_var1 = (i_var3 + 4);
        i_var2 = (i_var3 + 2) + local_5e * 10;
      // u_var4 = (param_5  >> 0x10);
      // u_var5 = (param_2_00  >> 0x10);
        if (((local_5c * 4 + param_5) == 0) && ((local_5c * 4 + param_2_00) == 0)) {
            break;
        }
        copy_string_1000_3d3e(CONCAT22(unaff_ss, local_54), *(local_5c * 4 + param_2_00));
      // u_var5 = (param_6  >> 0x10);
        (i_var2 + 4) = (local_5a * 2 + param_6);
        pass1_1040_a626(CONCAT22(u_var1, i_var2), CONCAT22(unaff_ss, local_54));
        string_fn_1000_3f9c(
            local_54,
            unaff_ss,
            s__lu_1050_393b,
            &ctx.g_alloc_addr_1050_1050,
            (param_5 + local_5c * 4),
        );
        u_var1 = (i_var3 + 4);
        i_var3 = (i_var3 + 2) + &local_5e.field_0x1 * 10;
        _local_58 = CONCAT22(u_var1, i_var3);
        (i_var3 + 4) = (&local_5a.field_0x1 * 2 + param_6);
        pass1_1040_a626(_local_58, CONCAT22(unaff_ss, local_54));
        local_5e = &local_5e.field_0x2;
        local_5a = &local_5a.field_0x2;
        local_5c = &local_5c.field_0x1;
    }
    return local_5e;
}

pub unsafe fn pass1_1010_dc36(
    param_1: u16,
    param_2: u16,
    param_3: u16,
    param_2_00: u32,
    param_5: &mut  u32,
) -> i32 {
    let pu_var1: &mut  u32;
    let mut u_var2: u32;
    let mut i_var3: i32;
    let mut u_var4: i32;
    let pu_var5: &mut  u32;
    let mut u_var6: u16;
    let mut unaff_ss: u16;
    let mut local_5a: u16;
    let mut local_58: u32;
    let string_1: String;
    let local_52: [u32; 20];

    string_1 = PTR_s_New_failed_in_Op_Op_1050_0020_1050_393f;
    i_var3 = 0x13;
    pu_var5 = local_52;
    while (i_var3 != 0) {
        i_var3 = i_var3 + -1;
        pu_var1 = pu_var5;
        pu_var5 = pu_var5 + 1;
        *pu_var1 = 0;
    }
    pu_var5 = 0;
    *(pu_var5 + 2) = 0;
    local_5a = param_1_00;
    loop {
      // u_var6 = (param_5  >> 0x10);
        if (*param_5 < local_5a || *param_5 == local_5a) {
            break;
        }
        u_var2 = (param_5 + 2);
        u_var4 = u_var2 + local_5a * 10;
        (u_var4 + 4) = (local_5a * 2 + param_2_00);
        local_5a = local_5a + 1;
        pass1_1040_a626(
            (u_var2 & 0xffff0000 | u_var4),
            CONCAT22(unaff_ss, &string_1),
        );
    }
    return;
}

pub unsafe fn pass1_1010_dcac(
    param_1: u16,
    param_2: u16,
    param_1_00: i32,
    param_2_00: u32,
    param_5: &mut  Struct503,
) -> i32 {
    let mut iVar1: i32;
    let mut i_var2: i32;
    let local_CX_130: &mut  Struct504;
    let local_bx_25: &mut  Struct503;
    let local_es_25: Vec<u8>;
    let local_es_60: Vec<u8>;
    let in_string_1: String;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_5f6227b039: &mut  Struct504;
    let temp_5fedbb8a6a: Vec<u8>;

    in_string_1 = load_string_1010_847e(
        ctx.g_struct_73_1050_14cc,
        (ctx.g_struct_73_1050_14cc >> 0x10),
        0x590,
    );
  // local_es_25 = (param_5  >> 0x10);
    local_bx_25 = param_5;
    temp_5fedbb8a6a = *&local_bx_25.field_0x4;
    iVar1 = local_bx_25.field_0x2 + param_1_00 * 10;
  // local_es_60 = (param_2_00  >> 0x10);
    (iVar1 + 4) = (param_1_00 * 2 + param_2_00);
    pass1_1040_a626(CONCAT22(temp_5fedbb8a6a, iVar1), in_string_1);
    copy_string_1000_3d3e(in_string_1, s__1050_3941);
    i_var2 = param_1_00 + 1;
    temp_5f6227b039 = &local_bx_25.field_0x2;
    local_CX_130 = temp_5f6227b039;
    local_CX_130 = (local_CX_130 + i_var2 * 10);
    local_CX_130.field_0x4 = (i_var2 * 2 + param_2_00);
    pass1_1040_a626(
        (temp_5f6227b039 & 0xffff0000 | ZEXT24(local_CX_130)),
        in_string_1,
    );
    return i_var2;
}

pub unsafe fn pass1_1010_dd5e(param_1: u32, param_1_00: &mut Struct493) {
    let mut iVar1: i32;
    let mut i_var2: i32;
    let mut u_var3: i32;
    let mut u_var4: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u32;
    let mut local_a: u16;
    let mut local_6: u32;

    if (param_1_00 != 0x0) {
        u_var4 = pass1_1030_73a8(param_1_00);
      // u_var3 = (u_var4  >> 0x10);
        i_var2 = u_var4;
        local_10 = u_var4 & 0xffff0000 | (i_var2 + 0x14);
        if ((u_var3 | i_var2 + 0x14) != 0) {
            iVar1 = (i_var2 + 0x12);
            i_var2 = (i_var2 + 0x18);
            if (((((iVar1 == 4)
                || (((iVar1 == 6 && (i_var2 == 4)) || (iVar1 == 5))
                    || (iVar1 == 6 && (i_var2 == 5))))
                || (iVar1 == 8))
                || (iVar1 == 6 && (i_var2 == 8)))
                && (local_10 != 0))
            {
                return;
            }
        }
    }
    return;
}

pub unsafe fn pass1_1010_ddf6(param_1: u32, param_2: u32) {
    let mut u_var1: u32;
    let mut in_resource_id: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    *(param_1 + 0x13c) = 0;
    u_var1 = pass1_1030_73a8(param_2);
    match (u_var1 + 0x12) {
        1 | 2 | 4 | 7 | 9 => in_resource_id = 0x598,
        3 | 5 => in_resource_id = 0x597,
        6 => in_resource_id = 0x599,
        8 => in_resource_id = 0x5f3,
        _ => {} // goto switchD_1010_de53_caseD_9;
    }
    load_string_1010_84e0(
        ctx.g_struct_73_1050_14cc,
        (ctx.g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        (param_1 & 0xffff0000 | (param_1 + 0x13c)),
        in_resource_id,
    );
    // switchD_1010_de53_caseD_9:
    return;
}

pub unsafe fn pass1_1010_debe(
    param_1: u32,
    param_2: u16,
    param_3: u32,
    param_4: u32,
    param_5: u32,
) -> i32 {
    let local_AX_8: &mut  Struct490;
    let mut u_var1: u16;
    let mut u_var2: i32;
    let mut u_var3: u16;
    let mut i_var4: i32;
    let pa_var5: &mut  Struct199;
    let struct_a: &mut  Struct199;
    let mut i32_var6: i32;
    let mut unaff_si: u16;
    let mut u_var7: u16;
    let mut u_var8: u32;
    let pp_var9: &mut  Struct2111;
    let mut u_var10: u16;
    let mut local_22: u16;
    let mut local_1e: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    param_4 = 0;
    param_3 = 0;
    u_var8 = pass1_1030_73a8(param_5);
    i_var4 = (u_var8 + 0x12);
    u_var3 = param_1;
  // u_var10 = (param_1  >> 0x10);
    u_var1 = pass1_1010_b028(u_var3, u_var10, u_var8);
    pp_var9 = pass8_funcs::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(unaff_si, 0x35));
  // pa_var5 = (pp_var9  >> 0x10);
    i32_var6 = param_4;
  // u_var7 = (param_4  >> 0x10);
    if (param_2 == 0x13) {
        local_22 = 0;
        while (local_22 = local_22 + 1, local_22 < 0x43) {
            u_var1 = pass1_1010_ac62(u_var3, u_var10, local_22);
            if (u_var1 != 0) {
                param_3 = param_3 + 1;
            }
        }
        u_var1 = param_3 * 2;
        process_struct_1000_179c(u_var1, pa_var5);
        param_4 = u_var1;
        (i32_var6 + 2) = pa_var5;
        if ((pa_var5 | param_4) != 0) {
            local_22 = 0;
            local_1e = 0;
            while (param_3 != local_1e && local_1e <= param_3) {
                while {
                    local_22 = local_22 + 1;
                    if (0x42 < local_22) {}
                    // goto LAB_1010_e0d4;
                    u_var1 = pass1_1010_ac62(u_var3, u_var10, local_22);
                    u_var1 == 0
                } {}
                (local_1e * 2 + param_4) = local_22;
                // LAB_1010_e0d4:
                local_1e = local_1e + 1;
            }
        }
    } else {
        if (param_2 < 0x14) {
            if (param_2 == '\x06') {
                if (((i_var4 == 5) || (i_var4 == 6)) || (i_var4 == 8)) {
                    u_var2 = pp_var9 + 0x11e;
                    u_var8 = pp_var9 & 0xffff0000 | u_var2;
                    if (u_var1 == 0xf) {
                        local_14 = 0xf;
                        local_16 = 0x13;
                    } else {
                        if (u_var1 == 0xe) {
                            local_16 = 4;
                            local_14 = 1;
                        } else {
                            local_16 = 0xe;
                            local_14 = 1;
                        }
                    }
                    pass1_1010_e128(u_var3, u_var10, local_16, local_14, u_var8);
                    i_var4 = u_var8 + 1;
                    param_3 = i_var4;
                    if (i_var4 != 0) {
                        u_var1 = param_3 * 2;
                        pa_var5 = struct_a;
                        process_struct_1000_179c(u_var1, struct_a);
                        param_4 = u_var1;
                        (i32_var6 + 2) = pa_var5;
                        local_18 = 0;
                        local_1a = local_14;
                        while (local_1a <= local_16) {
                            if ((local_1a * 2 + u_var2) != 0) {
                                (param_4 + local_18 * 2) = local_1a;
                                local_18 = local_18 + 1;
                            }
                            local_1a = local_1a + 1;
                        }
                        (param_4 + local_18 * 2) = 0x14;
                        return;
                    }
                }
            } else {
                if ((param_2 == 0x7) && ((i_var4 == 5 || (i_var4 == 6)) || (i_var4 == 8))) {
                    u_var3 = pass1_1010_ac62(u_var3, u_var10, 7);
                    i_var4 = -(u_var3 == 0) + 0x10;
                    param_3 = i_var4;
                    u_var1 = i_var4 * 2;
                    process_struct_1000_179c(u_var1, pa_var5);
                    param_4 = u_var1;
                    (i32_var6 + 2) = pa_var5;
                    if ((pa_var5 | param_4) == 0) {
                        param_3 = 0;
                        return;
                    }
                    local_1a = 0;
                    while (local_1a < (-(u_var3 == 0) + 0xf)) {
                        (local_1a * 2 + param_4) = local_1a + 1;
                        local_1a = local_1a + 1;
                    }
                    (local_1a * 2 + param_4) = 0x10;
                    return;
                }
            }
        }
    }
    return;
}

pub unsafe fn pass1_1010_e128(
    param_1: u16,
    param_2: u16,
    param_1_00: i32,
    param_4: u16,
    param_5: u32,
) -> i32 {
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 0;
    local_6 = param_2_00;
    while (local_6 <= param_1_00) {
        if ((local_6 * 2 + param_5) != 0) {
            local_4 = local_4 + 1;
        }
        local_6 = local_6 + 1;
    }
    return local_4;
}

pub unsafe fn pass1_1010_e15e(param_1: u32) {
    let pp_var1: fn();

    let mut u_var2: u16;
    let paVar3: &mut  Struct493;
    let mut u_var4: u32;



    let mut u_var5: u16;
    let mut unaff_cs: u16;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    pass1_1010_afde(param_1, 0xc);
    _local_a = CONCAT22(ctx.dx_reg, in_ax);
    pp_var1 = (*_local_a + 0x10);
    u_var2 = in_ax;
    u_var6 = in_ax;
    u_var7 = ctx.dx_reg;
    (**pp_var1)();
    _local_e = CONCAT22(ctx.dx_reg, u_var2);
    local_12 = 0;
    while (local_12 < _local_e) {
        pp_var1 = (*_local_a + 4);
        u_var4 = _local_e;
        (**pp_var1)(unaff_cs, in_ax, ctx.dx_reg, local_12, (local_12 >> 0x10));
        u_var5 = ctx.dx_reg;
        paVar3 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var4, ctx.dx_reg);
        unaff_cs = 0x1030;
        pass1_1030_7c28(CONCAT13((u_var5 >> 8), CONCAT12(u_var5, paVar3)), 0x23);
        local_12 = local_12 + 1;
    }
    if (_local_a != 0x0) {
        pp_var1 = *_local_a;
        (**pp_var1)(unaff_cs, in_ax, ctx.dx_reg, 1, u_var6, u_var7);
    }
    return;
}

pub unsafe fn pass1_1010_ec18(param_1: u16, param_2: u16, param_1_00: u32) {
    let mut local_DXAX_13: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    local_DXAX_13._0_2_ =
        pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, param_1_00, (param_1_00 >> 0x10));
    return CONCAT22(&(local_DXAX_13).field_0x12, &(local_DXAX_13).field_0x10);
}

pub unsafe fn pass1_1010_ec40(param_1: u16, param_2: u16, param_1_00: u32) {
    let mut local_DXAX_13: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    local_DXAX_13._0_2_ =
        pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, param_1_00, (param_1_00 >> 0x10));
    return CONCAT22(&(local_DXAX_13).field_0x12, &(local_DXAX_13).field_0x10);
}

pub unsafe fn pass1_1010_ec68(param_1: &mut  Struct318, param_2: u32) {
    let mut u_var1: i32;

  // u_var1 = (param_1  >> 0x10);
    (param_1 + 0x28) = param_2;
    pass8_funcs::pass1_1010_1f62((param_1 & 0xffff | u_var1 << 0x10), 0x13);
    return;
}

pub unsafe fn pass1_1010_ec84(param_1: &mut  Struct318) {
    let mut unaff_ss: u16;
    let mut local_10e: u16;
    let mut local_10c: u16;

    pass8_funcs::pass1_1010_1f62(param_1, 0x14);
    pass1_1030_532e(CONCAT22(unaff_ss, &local_10e), (param_1 + 0x20));
    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_10e));
    return;
}

pub unsafe fn pass1_1010_ecc6(param_1: u32, param_2: u32, param_3: u32) {
    let mut u_var1: u32;
    let ctx.ax_reg: &mut  Struct505;
    let paVar2: &mut  Struct493;

    let mut u_var3: u16;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;

    pass1_1030_627e(_PTR_LOOP_1050_5740, param_2, param_3);
    u_var3 = ctx.dx_reg;
    paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, ctx.ax_reg, ctx.dx_reg);
    u_var1 = &paVar2[1].field_0x10;
  // u_var5 = (u_var1  >> 0x10);
    i_var4 = u_var1;
    if ((i_var4 + 0x200) == 0x8000001) {
        pass1_1010_ed22(param_1, (i_var4 + 4));
    }
    return;
}

pub unsafe fn pass1_1010_ed22(param_1: u32, param_2: u32) {
    let mut u_var1: i32;

  // u_var1 = (param_1  >> 0x10);
    (param_1 + 0x24) = param_2;
    pass8_funcs::pass1_1010_1f62((param_1 & 0xffff | u_var1 << 0x10), 0x12);
    return;
}

pub unsafe fn pass1_1010_ed3e(param_1: u32) {
    let mut u_var1: u32;

    u_var1 = (param_1 + 0x16);
    pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
    return;
}

pub unsafe fn pass1_1018_017c(param_1: u32, param_2: u16) {
    let mut u_var1: i32;

  // u_var1 = (param_1  >> 0x10);
    (param_1 + 0x1e) = param_2;
    pass8_funcs::pass1_1010_1f62((param_1 & 0xffff | u_var1 << 0x10), 4);
    return;
}

pub unsafe fn pass1_1018_0196(param_1: &mut  Struct507, param_2: u32, param_3: u32) {
    let piVar1: &mut  i32;
    let mut i_var2: i32;
    let mut u_var3: u32;
    let ppVar4: &mut  pass1_struct_2;
    let in_dx: &mut  u16;
    let ctx.dx_reg: &mut  u16;
    let local_bx_18: &mut  Struct507;
    let mut u_var5: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_1e: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_e: u16;
    let mut local_c: u16;

    g_struct_ptr_1 = pass1_1030_8344(
        ctx._g_bool_1050_5748,
        (ctx._g_bool_1050_5748 >> 0x10),
        param_3,
    );
  // u_var5 = (param_1  >> 0x10);
    local_bx_18 = param_1;
    if (&local_bx_18.field_0x2c == 0) {
        local_bx_18.field_0x32 = 5;
        if (ctx.g_struct_ptr_1 == 0) {
            struct_fn_1000_160a();
            ctx.g_u16_ptr_1050_5f2e = in_dx;
        } else {
        }
        alloc_mem_1000_1708(0x1e, 0x10000, g_struct_ptr_1, ctx.g_u16_ptr_1050_5f2e);
    } else {
        ctx.g_u16_ptr_1050_5f2e = in_dx;
        if (local_bx_18.field_0x30 + 1 < local_bx_18.field_0x32) {}
        // goto LAB_1018_022a;
        piVar1 = &local_bx_18.field_0x32;
        *piVar1 = *piVar1 + 5;
        g_struct_ptr_1 = (local_bx_18.field_0x32 * 6);
        alloc_mem_1000_0ed4(1, g_struct_ptr_1, 0, &local_bx_18.field_0x2c);
        ctx.g_u16_ptr_1050_5f2e = ctx.dx_reg;
    }
    local_bx_18.field_0x2c = g_struct_ptr_1;
    &local_bx_18.field_0x2e = ctx.g_u16_ptr_1050_5f2e;
    // LAB_1018_022a:
    ppVar4 = pass1_1030_8344(
        ctx._g_bool_1050_5748,
        (ctx._g_bool_1050_5748 >> 0x10),
        param_2,
    );
    ppVar4 = pass1_1030_8344(
        ctx._g_bool_1050_5748,
        (ctx._g_bool_1050_5748 >> 0x10),
        &ppVar4.field_0x10,
    );
    i_var2 = local_bx_18.field_0x30;
    piVar1 = &local_bx_18.field_0x30;
    *piVar1 = *piVar1 + 1;
    u_var3 = &local_bx_18.field_0x2c;
    modify_list_1008_3f62(
        (u_var3 & 0xffff0000 | (u_var3 + i_var2 * 6)),
        CONCAT22(ctx.g_u16_ptr_1050_5f2e, &ppVar4.field_0xc),
    );
    return;
}

pub unsafe fn pass1_1018_028c(param_1: u32, param_2: u32) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let pu_var3: Vec<u8>;
    let mut u_var4: i32;
    let mut u_var5: i32;
    let mut i32_var6: i32;
    let mut u_var7: u32;
    let mut in_dx: u16;
    let mut u_var8: i32;
    let struct_a: &mut  Struct199;
    let paVar9: &mut  Struct199;



    let mut iVar10: i32;
    let mut u_var11: u16;
    let mut u_var12: u16;
    let mut unaff_ss: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: [u8; 4];
    let mut local_14: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_6 = pass1_1030_8344(
        ctx._g_bool_1050_5748,
        (ctx._g_bool_1050_5748 >> 0x10),
        param_2,
    );
    local_8 = pass1_1030_5b00(CONCAT22(in_dx, local_6));
    _local_c = pass8_funcs::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(local_32, local_8));
    zero_list_1008_6c90(local_18, unaff_ss);
    pass1_1018_0b1e(_local_c, (_local_c >> 0x10), local_18, unaff_ss);
    u_var8 = local_14 >> 0xf;
    if ((u_var8 | local_14) == 0) {
        pu_var3 = local_18;
        pass1_1030_6522(_PTR_LOOP_1050_5740, CONCAT22(unaff_ss, pu_var3), param_2);
    } else {
        pu_var3 = local_18;
        pass1_1030_62e4(_PTR_LOOP_1050_5740, CONCAT22(unaff_ss, pu_var3), param_2);
    }
    _local_1c = CONCAT22(u_var8, pu_var3);
    u_var4 = u_var8 | pu_var3;
    if (u_var4 == 0) {
        return;
    }
    pass1_1018_04f2(param_1);
    u_var12 = 0x1000;
    paVar9 = struct_a;
    process_struct_1000_179c(0x1c, struct_a);
    iVar10 = param_1;
  // u_var11 = (param_1  >> 0x10);
    u_var5 = u_var4;
    if ((paVar9 | u_var4) == 0) {
        (iVar10 + 0x12) = 0;
    } else {
        u_var12 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
        process_struct_1008_8e9e(CONCAT22(paVar9, u_var4), 6, 0x24);
        (iVar10 + 0x12) = u_var5;
        (iVar10 + 0x14) = ctx.dx_reg;
    }
    ppc_var2 = (*_local_1c + 0x10);
    ppc_var2(u_var12, pu_var3, u_var8, u_var4);
    local_24 = 0;
    while (local_24 < u_var5) {
        u_var7 = SEXT24(local_24);
        ppc_var2 = (*_local_1c + 4);
        ppc_var2();
        if ((ctx.dx_reg | u_var7) != 0) {
            i32_var6 = local_24 / 6;
            u_var1 = (iVar10 + 0xe);
            pass1_1018_dd7c(
                u_var1,
                (u_var1 >> 0x10),
                CONCAT22(local_24 % 6, i32_var6),
                (u_var7 & 0xffff | ctx.dx_reg << 0x10),
            );
            pass1_1008_8faa((iVar10 + 0x12), CONCAT22(ctx.dx_reg, i32_var6));
        }
        local_24 = local_24 + 1;
    }
    return;
}

pub unsafe fn pass1_1018_03ea(param_1: u32, param_2: i32) {
    if (param_2 != 5) {
        return;
    }
    pass8_funcs::pass1_1010_1f62((param_1 & 0xffff0000 | (param_1 - 10)), 5);
    return;
}

pub unsafe fn pass1_1018_0412(
    param_1: u32,
    param_2: u16,
    param_3: u32,
    param_4: u16,
    param_5: u32,
) -> i32 {
    let mut iVar1: i32;
    let mut unaff_ss: u16;
    let mut local_12e: u16;
    let mut local_12c: u16;
    let mut local_12a: u16;
    let mut local_128: u16;
    let mut local_126: u16;
    let mut local_4: u16;

    local_4 = 0;
    if (((0x72 < param_4) && (!SBORROW2(param_4, 0x73)))
        && (param_4 == 0x75
            || (param_4 - 0x74) < 1
            || (0 < (param_4 - 0x76) && ((param_4 - 0x77) < 2))))
    {
        local_4 = 1;
    }
    pass1_1028_933c(
        CONCAT22(unaff_ss, &local_128),
        param_2,
        local_4,
        param_4,
        param_3,
        (param_3 >> 0x10),
        (param_1 + 0x24),
        param_5,
    );
    iVar1 = pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_128));
    if (iVar1 != 0) {
        pass8_funcs::pass1_1010_1f62(param_1, 6);
    }
    return;
}

pub unsafe fn pass1_1018_04a4(param_1: u32, param_2: u32) {
    (param_1 + 0x16) = param_2;
    return;
}

pub unsafe fn pass1_1018_04b8(param_1: u32) {
    let mut u_var1: u16;

  // u_var1 = (param_1  >> 0x10);
    return CONCAT22((param_1 + 0x18), (param_1 + 0x16));
}

pub unsafe fn pass1_1018_04ca(param_1: u32, param_2: u32) {
    (param_1 + 0x1a) = param_2;
    return;
}

pub unsafe fn pass1_1018_04de(param_1: u32, param_2: u32) {
    (param_1 + 0x20) = param_2;
    return;
}

pub unsafe fn pass1_1018_04f2(param_1: u32) {
    let pu_var1: &mut  u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let local_bx_4: &mut  Struct498;
    let mut local_es_4: u16;
    let temp_8621d6c8635: &mut  u32;

  // local_es_4 = (param_1  >> 0x10);
    local_bx_4 = param_1;
    pu_var1 = local_bx_4.field_0x12;
    u_var2 = &local_bx_4.field_0x14;
    if ((u_var2 | pu_var1) != 0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    &local_bx_4.field_0x12 = 0;
    return;
}

pub unsafe fn pass1_1018_0532(param_1: u32, param_2: u8) {
    pass1_1010_eb66(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1018_078e(param_1: &mut  Struct376) {
    let mut u_var1: i32;
    let p_uvar2: &mut  u16;
    let mut u_var3: i32;
    let local_bx_5: &mut  Struct376;
    let mut u_var4: i32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_6: u32;

  // u_var4 = (param_1  >> 0x10);
    local_bx_5 = param_1;
    param_1.ptr_a_lo = (s_582_bmp_1050_1871 + 3);
    local_bx_5.ptr_a_hi = 0x1018;
    local_bx_5.ptr_2_lo = (s_589_bmp_1050_18a9 + 7);
    local_bx_5.ptr_2_hi = 0x1018;
    PTR_LOOP_1050_3960 = PTR_LOOP_1050_3960 + -1;
    (_PTR_LOOP_1050_3962 + local_bx_5.palette_handle_x12 * 2 + -4) = 0;
    if (PTR_LOOP_1050_3960 == 0x0) {
        error_check_1000_17ce(_PTR_LOOP_1050_3962);
        _PTR_LOOP_1050_3962 = 0x0;
    }
    error_check_1000_17ce(local_bx_5.u32_x94);
    error_check_1000_17ce(local_bx_5.u32_x9a);
    error_check_1000_17ce(local_bx_5.u32_x88);
    error_check_1000_17ce(local_bx_5.u32_x8e);
    if (local_bx_5.u32_x2c != 0) {
        if ((u_var4 | local_bx_5) == 0) {
            pu_var2 = 0x0;
            u_var3 = 0;
        } else {
            pu_var2 = &local_bx_5.ptr_2_lo;
            u_var3 = u_var4;
        }
        pass8_funcs::pass1_1010_1ea6(local_bx_5.u32_x2c, CONCAT22(u_var3, pu_var2));
    }
    if (local_bx_5.u32_x9e != 0) {
        if ((u_var4 | local_bx_5) == 0) {
            pu_var2 = 0x0;
            u_var3 = 0;
        } else {
            pu_var2 = &local_bx_5.ptr_2_lo;
            u_var3 = u_var4;
        }
        pass8_funcs::pass1_1010_1ea6(local_bx_5.u32_x9e, CONCAT22(u_var3, pu_var2));
    }
    u_var3 = local_bx_5.u16_x60;
    u_var1 = local_bx_5.u16_x62;
    local_6 = CONCAT22(u_var1, u_var3);
    if ((u_var1 | u_var3) != 0) {
        pass1_1008_5118(CONCAT22(u_var1, u_var3));
        error_check_1000_17ce(local_6);
    }
    local_bx_5.u32_x4c = 0;
    if (param_1 == 0x0) {
        pu_var2 = 0x0;
        u_var4 = 0;
    } else {
        pu_var2 = &local_bx_5.ptr_2_lo;
    }
    _local_1a = CONCAT22(u_var4, pu_var2);
    *_local_1a = ctx.s_1_1050_389a;
    pu_var2[1] = &ctx.PTR_LOOP_1050_1008;
    pass8_funcs::pass1_1010_1d80(param_1);
    return;
}

pub unsafe fn pass1_1018_0902(param_1: &mut  u32, param_2: u32) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let ppVar3: &mut  pass1_struct_2;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut u_var6: u32;
    let pu_var7: &mut  u32;
    let puVar8: &mut  u32;
    let mut local_8: u16;

    puVar8 = (param_1 & 0xffff0000 | (param_1 + 0x28));
    pu_var7 = (param_1 & 0xffff0000 | ZEXT24((param_1 + 0x24)));
    u_var5 = param_1;
    ppVar3 = pass1_1030_8344(
        ctx._g_bool_1050_5748,
        (ctx._g_bool_1050_5748 >> 0x10),
        param_2,
    );
    pass1_1030_5a52(CONCAT22(u_var5, ppVar3), pu_var7, puVar8);
    u_var6 = process_struct_1008_4772((param_1 + 0x24));
    i_var4 = u_var6;
    (param_1 + 0x5a) = i_var4;
    (param_1 + 0x5c) = (u_var6 >> 0x10);
    pass1_1018_17f0(param_1);
    (param_1 + 0x12) = i_var4 + 2;
    (i_var4 * 2 + _PTR_LOOP_1050_3962) = 1;
    ppc_var2 = (*param_1 + 0x18);
    ppc_var2();
    (param_1 + 0x3c) = param_2;
    u_var1 = (param_1 + 0x2c);
    u_var6 = pass1_1010_ec18(
        u_var1,
        (u_var1 >> 0x10),
        param_2 & 0xffff0000 | *(param_1 + 0x3c),
    );
    (param_1 + 0x7c) = u_var6;
    (param_1 + 0x7e) = (u_var6 >> 0x10);
    return;
}

pub unsafe fn pass1_1018_0a50(param_1: &mut  Struct510) {
    let local_bx_3: &mut  Struct510;
    let mut u_var1: u16;

  // u_var1 = (param_1  >> 0x10);
    local_bx_3 = param_1;
    if (local_bx_3.field_0x84 == 2) {
        return CONCAT22(local_bx_3.field_0x2a, local_bx_3.field_0x28);
    }
    return CONCAT22(local_bx_3.field_0x26, local_bx_3.field_0x24);
}

pub unsafe fn pass1_1018_0a76(param_1: u32) {
    let mut u_var1: u16;
    let mut u_var2: i32;

  // u_var2 = (param_1  >> 0x10);
    if ((param_1 + 0x84) == 1) {
        u_var1 = 2;
    } else {
        u_var1 = 1;
    }
    (param_1 + 0x84) = u_var1;
    pass8_funcs::pass1_1010_1f62((param_1 & 0xffff | u_var2 << 0x10), 4);
    return;
}

pub unsafe fn pass1_1018_0aa0(param_1: &mut  Struct511, param_2: u16) {
    let local_bx_6: &mut  Struct511;
    let mut u_var1: u16;

  // u_var1 = (param_1  >> 0x10);
    local_bx_6 = param_1;
    local_bx_6.field_0x14 = param_2;
    pass1_1018_04de(local_bx_6.field_0x2c, local_bx_6.field_0x3c);
    return;
}

pub unsafe fn pass1_1018_0ac0(param_1: u32, param_2: u32) {
    (param_1 + 0x80) = param_2;
    return;
}

pub unsafe fn pass1_1018_0ad4(param_1: u32) {
    let mut u_var1: u16;

  // u_var1 = (param_1  >> 0x10);
    return CONCAT22((param_1 + 0x82), (param_1 + 0x80));
}

pub unsafe fn pass1_1018_0ae8(param_1: u32, param_2: u16) {
    (param_1 + 0x5e) = param_2;
    return;
}

pub unsafe fn pass1_1018_0afa(param_1: u32) {
    return (param_1 + 0x5e);
}

pub unsafe fn pass1_1018_0b08(param_1: u32) {
    let mut u_var1: u32;
    let local_bx_6: &mut  Struct512;
    let mut u_var2: u16;

    u_var1 = (param_1 + 0x7c);
  // u_var2 = (u_var1  >> 0x10);
    local_bx_6 = u_var1;
    return CONCAT22(local_bx_6.field_0x6, local_bx_6.field_0x4);
}

pub unsafe fn pass1_1018_0b1e(param_1: u32, param_2: &mut  u16) {
    let mut iVar1: i32;
    let mut u_var2: u32;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    i_var3 = param_1;
    pass1_1008_3eb4(
        (param_1 & 0xffff0000 | (i_var3 + 0x30)),
        CONCAT22(unaff_ss, &local_8),
        CONCAT22(unaff_ss, &local_6),
        CONCAT22(unaff_ss, &local_4),
    );
    if ((local_4 - 3) < 1) {
        local_4 = 3;
    }
    if ((local_6 - 3) < 1) {
        local_6 = 3;
    }
  // u_var4 = (param_1  >> 0x10);
    u_var2 = (i_var3 + 0x5a);
    iVar1 = (u_var2 + 4);
    if (iVar1 <= (local_4 + 2)) {
        local_4 = iVar1 - 3;
    }
    u_var2 = (i_var3 + 0x5a);
    iVar1 = (u_var2 + 8);
    if (iVar1 <= (local_6 + 2)) {
        local_6 = iVar1 - 3;
    }
    pass1_1008_6cec(
        param_1 & 0xffff0000 | (i_var3 + 0x40),
        local_8,
        CONCAT22(local_4 + 2, local_6 + 2),
        local_8,
        CONCAT22(local_4 - 3, local_6 - 3),
    );
    modify_list_1008_3f62(param_2, param_1 & 0xffff0000 | (i_var3 + 0x40));
    modify_list_1008_3f62(
        (param_2 & 0xffff0000 | (param_2 + 6)),
        param_1 & 0xffff0000 | (i_var3 + 0x46),
    );
    return;
}
