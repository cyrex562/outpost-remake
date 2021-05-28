use crate::bad_funcs::bad1::halt_baddata;
use crate::err_ops::error_check_1000_17ce;
use crate::other_funcs::mixed_fn_1010_830a;
use crate::pass::{pass12_funcs, pass14_funcs, pass_funcs};
use crate::pass::pass15_funcs::{pass1_1020_a43e, pass1_1020_a6ee};
use crate::pass::pass17_funcs::{pass1_1030_38f2, pass1_1030_4bbe};
use crate::pass::pass7_funcs::{pass1_1018_4842, pass1_1018_4cda, pass1_1018_4dce};
use crate::pass::pass8_funcs::pass1_1010_1d80;
use crate::struct_ops::struct_ops_2::{process_struct_1000_179c, process_struct_1008_574a, process_struct_1008_c882, process_struct_1010_1d48};
use crate::structs::prog_structs_17::Struct534;
use crate::structs::prog_structs_27::{Struct298, Struct301};
use crate::structs::prog_structs_29::Struct299;
use crate::structs::prog_structs_2::Struct199;
use crate::structs::prog_structs_30::{Struct294, Struct295};
use crate::structs::prog_structs_7::Struct44;
use crate::ui_ops::misc::win_cleanup_1018_4d22;
use crate::util::{CARRY1, CONCAT22};

pub unsafe fn pass1_1008_c79a(param_1: u32, param_2: u32) {
    let mut u_var1: u32;
    let pu_var2: Vec<u8>;
    let mut u_var3: u32;


    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_12: [u8; 4];
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

  // u_var5 = (param_1  >> 0x10);
    pass14_funcs::pass1_1008_5784(CONCAT22(unaff_ss, local_a), (param_1 + 10));
    while (true) {
        pu_var2 = local_a;
        pass14_funcs::pass1_1008_5b12(CONCAT22(unaff_ss, pu_var2));
        _local_e = CONCAT22(ctx.dx_reg, pu_var2);
        if ((ctx.dx_reg | pu_var2) == 0) {
            break;
        }
        u_var1 = (pu_var2 + 4);
        pass_funcs::pass1_1000_3d7a(u_var1, (u_var1 >> 0x10), param_2);
        if (pu_var2 == 0x0) {
            pass1_1020_a43e(CONCAT22(unaff_ss, local_12));
            pass1_1020_a6ee(CONCAT22(unaff_ss, local_12), (_local_e + 0x12));
            u_var3 = (ctx._PTR_LOOP_1050_65e2 + 0x52);
            u_var4 = ctx.dx_reg;
            pass1_1030_4bbe(u_var3, (_local_e + 0x12));
            (param_1 + 0xe) = (u_var3 + 0x94) + *ctx._PTR_LOOP_1050_65e2;
        }
    }
    return;
}

pub unsafe fn pass1_1008_c83a(param_1: u32) {
    if (*ctx._PTR_LOOP_1050_65e2 <= (param_1 + 0xe)) {
        return;
    }
    return;
}

pub unsafe fn pass1_1008_c85e(param_1: u32) {
    let local_bx_3: &mut  Struct294;
    let mut u_var1: i32;

  // u_var1 = (param_1  >> 0x10);
    local_bx_3 = param_1;
    if (&local_bx_3.field_0xa == 0) {
        process_struct_1008_c882(param_1 & 0xffff | u_var1 << 0x10);
    }
    return CONCAT22(local_bx_3.field_0xc, local_bx_3.field_0xa);
}

pub unsafe fn pass1_1008_ca24(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass12_funcs::pass1_1008_c75c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_ca5a(param_1: &mut  Struct295, param_2: u32) {
    let mut u_var1: u16;

    u_var1 = param_2;
    process_struct_1010_1d48(CONCAT22(u_var1, param_1), (param_2 >> 0x10));
    param_1.field_0xa = 0;
    param_1.field_0xe = 0;
    param_1.field_0x12 = 0;
    param_1.field_0x16 = 0;
    param_1.field_0x1a = 0;
    param_1.field_0x1e = 0;
    CONCAT22(u_var1, param_1) = 0xd71a;
    param_1.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub unsafe fn pass1_1008_caa0(param_1: &mut  u16) {
    let mut u_var1: i32;

  // u_var1 = (param_1  >> 0x10);
    unsafe { *param_1 = 0xd71a };
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    pass1_1008_cac6(param_1 & 0xffff | u_var1 << 0x10);
    pass1_1010_1d80(param_1);
    return;
}

pub unsafe fn pass1_1008_cac6(param_1: u32) {
    let pu_var1: &mut  u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let mut i_var4: i32;
    let mut u_var5: u16;

  // u_var5 = (param_1  >> 0x10);
    i_var4 = param_1;
    pu_var1 = (i_var4 + 10);
    u_var2 = (i_var4 + 0xc);
    if ((u_var2 | pu_var1) != 0) {
        unsafe { ppc_var3 = *pu_var1 };
        (**ppc_var3)();
    }
    (i_var4 + 10) = 0;
    pu_var1 = (i_var4 + 0xe);
    u_var2 = (i_var4 + 0x10);
    if ((u_var2 | pu_var1) != 0) {
        unsafe { ppc_var3 = *pu_var1 };
        (**ppc_var3)();
    }
    (i_var4 + 0xe) = 0;
    pu_var1 = (i_var4 + 0x12);
    u_var2 = (i_var4 + 0x14);
    if ((u_var2 | pu_var1) != 0) {
        unsafe { ppc_var3 = *pu_var1 };
        (**ppc_var3)();
    }
    (i_var4 + 0x12) = 0;
    pu_var1 = (i_var4 + 0x16);
    u_var2 = (i_var4 + 0x18);
    if ((u_var2 | pu_var1) != 0) {
        unsafe { ppc_var3 = *pu_var1 };
        (**ppc_var3)();
    }
    (i_var4 + 0x16) = 0;
    pu_var1 = (i_var4 + 0x1a);
    u_var2 = (i_var4 + 0x1c);
    if ((u_var2 | pu_var1) != 0) {
        unsafe { ppc_var3 = *pu_var1 };
        (**ppc_var3)();
    }
    (i_var4 + 0x1a) = 0;
    pu_var1 = (i_var4 + 0x1e);
    u_var2 = (i_var4 + 0x20);
    if ((u_var2 | pu_var1) != 0) {
        unsafe { ppc_var3 = *pu_var1 };
        (**ppc_var3)();
    }
    (i_var4 + 0x1e) = 0;
    return;
}

pub unsafe fn pass1_1008_cfa0(param_1: &mut  Struct298, param_2: u32) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut b_var3: bool;
    let pu_var4: &mut  u32;
    let pa_var5: &mut  Struct199;
    let mut u_var6: i32;
    let mut u_var7: i32;
    let mut u_var8: u32;

    let mut u_var9: i32;

    let struct_a: &mut  Struct199;


    let struct_a_00: &mut  Struct199;

    let mut u_var10: u16;
    let mut extraout_dx_04: i32;
    let struct_a_01: &mut  Struct199;
    let mut extraout_dx_05: u16;
    let struct_a_02: &mut  Struct199;
    let mut extraout_dx_06: u16;
    let mut u_var11: u16;
    let mut i_var12: i32;
    let mut u_var13: u16;
    let u_var14: u8;
    let mut uVar15: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_16: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let mut local_4: u16;

  // u_var13 = (param_1  >> 0x10);
    i_var12 = param_1;
    pu_var4 = (i_var12 + 0x16);
    pa_var5 = (i_var12 + 0x18);
    if ((pa_var5 | pu_var4) != 0) {
        unsafe { ppc_var2 = *pu_var4 };
        ppc_var2();
        pa_var5 = ctx.dx_reg;
    }
    process_struct_1000_179c(0xc, pa_var5);
    u_var9 = pa_var5 | pu_var4;
    if (u_var9 == 0) {
        pa_var5 = 0x0;
        u_var9 = 0;
    } else {
        pa_var5 = process_struct_1008_574a(CONCAT22(pa_var5, pu_var4));
    }
    (i_var12 + 0x16) = pa_var5;
    (i_var12 + 0x18) = u_var9;
    b_var3 = false;
    u_var8 = (param_2 + 0x1f6);
    u_var11 = u_var8;
  // uVar15 = (u_var8  >> 0x10);
    pass1_1030_38f2(u_var11, uVar15, 2);
    u_var9 = u_var8;
    if ((-1 < ctx.dx_reg) && (0 < ctx.dx_reg || (u_var9 != 0))) {
        u_var7 = u_var9;
        load_str_1010_84ac(
            ctx.g_struct_73_1050_14cc,
            (ctx.g_struct_73_1050_14cc >> 0x10),
            0x430,
        );
        u_var14 = 0;
        pa_var5 = struct_a;
        u_var9 = u_var7;
        process_struct_1000_179c(0x14, struct_a);
        if ((pa_var5 | u_var9) == 0) {
            u_var9 = 0;
            u_var10 = 0;
        } else {
            u_var14 = 0x18;
            pass1_1018_4842(
                CONCAT22(pa_var5, u_var9),
                u_var8 & 0xffff | ctx.dx_reg << 0x10,
                CONCAT22(struct_a, u_var7),
                2,
            );
            u_var10 = ctx.dx_reg;
        }
        u_var1 = (i_var12 + 0x16);
        ppc_var2 = ((i_var12 + 0x16) + 4);
        ppc_var2(u_var14, u_var1, (u_var1 >> 0x10), u_var9, u_var10);
        b_var3 = true;
    }
    pass1_1030_38f2(u_var11, uVar15, 3);
    if ((-1 < ctx.dx_reg) && (0 < ctx.dx_reg || (u_var9 != 0))) {
        u_var6 = u_var9;
        load_str_1010_84ac(
            ctx.g_struct_73_1050_14cc,
            (ctx.g_struct_73_1050_14cc >> 0x10),
            0x431,
        );
        u_var14 = 0;
        pa_var5 = struct_a_00;
        u_var7 = u_var6;
        process_struct_1000_179c(0x14, struct_a_00);
        if ((pa_var5 | u_var7) == 0) {
            u_var9 = 0;
            u_var10 = 0;
        } else {
            u_var14 = 0x18;
            u_var8 = CONCAT22(ctx.dx_reg, u_var9);
            u_var9 = u_var7;
            pass1_1018_4842(
                CONCAT22(pa_var5, u_var7),
                u_var8,
                CONCAT22(struct_a_00, u_var6),
                3,
            );
            u_var10 = ctx.dx_reg;
        }
        u_var1 = (i_var12 + 0x16);
        ppc_var2 = ((i_var12 + 0x16) + 4);
        ppc_var2(u_var14, u_var1, (u_var1 >> 0x10), u_var9, u_var10);
        b_var3 = true;
    }
    pass1_1030_38f2(u_var11, uVar15, 4);
    if ((-1 < extraout_dx_04) && (0 < extraout_dx_04 || (u_var9 != 0))) {
        u_var6 = u_var9;
        load_str_1010_84ac(
            ctx.g_struct_73_1050_14cc,
            (ctx.g_struct_73_1050_14cc >> 0x10),
            0x432,
        );
        u_var14 = 0;
        pa_var5 = struct_a_01;
        u_var7 = u_var6;
        process_struct_1000_179c(0x14, struct_a_01);
        if ((pa_var5 | u_var7) == 0) {
            u_var9 = 0;
            u_var11 = 0;
        } else {
            u_var14 = 0x18;
            u_var8 = CONCAT22(extraout_dx_04, u_var9);
            u_var9 = u_var7;
            pass1_1018_4842(
                CONCAT22(pa_var5, u_var7),
                u_var8,
                CONCAT22(struct_a_01, u_var6),
                4,
            );
            u_var11 = extraout_dx_05;
        }
        u_var1 = (i_var12 + 0x16);
        ppc_var2 = ((i_var12 + 0x16) + 4);
        ppc_var2(u_var14, u_var1, (u_var1 >> 0x10), u_var9, u_var11);
        b_var3 = true;
    }
    if (!b_var3) {
        load_str_1010_84ac(
            ctx.g_struct_73_1050_14cc,
            (ctx.g_struct_73_1050_14cc >> 0x10),
            0x440,
        );
        u_var14 = 0;
        pa_var5 = struct_a_02;
        u_var7 = u_var9;
        process_struct_1000_179c(0x14, struct_a_02);
        if ((pa_var5 | u_var7) == 0) {
            u_var7 = 0;
            u_var11 = 0;
        } else {
            u_var14 = 0x18;
            pass1_1018_4842(
                CONCAT22(pa_var5, u_var7),
                0,
                CONCAT22(struct_a_02, u_var9),
                0,
            );
            u_var11 = extraout_dx_06;
        }
        u_var1 = (i_var12 + 0x16);
        ppc_var2 = ((i_var12 + 0x16) + 4);
        ppc_var2(u_var14, u_var1, (u_var1 >> 0x10), u_var7, u_var11);
    }
    return;
}

pub unsafe fn pass1_1008_d6f4(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1008_caa0(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_d72e(param_1: i32, param_2: u16, param_3: u16) {
    process_struct_1010_1d48(CONCAT22(param_2, param_1), param_3);
    (param_1 + 10) = 0;
    CONCAT22(param_2, param_1) = 0xd780;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    return CONCAT22(param_2, param_1);
}

pub unsafe fn pass1_1008_d75a(param_1: u32, param_2: u8) {
    pass1_1010_1d80(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_d790(in_struct_1: &mut  Struct299, param_2: u32) {
    let mut local_AX_18: u16;
    let mut local_DX_49: u16;
    let mut local_6: u16;
    let mut uStack4: u16;

    local_6 = param_2;
  // uStack4 = (param_2  >> 0x10);
    process_struct_1010_1d48(CONCAT22(local_6, in_struct_1), uStack4);
    local_AX_18 = 0;
    &in_struct_1.field_0xa = 0;
    in_struct_1.field_0xe = 0;
    CONCAT22(local_6, in_struct_1) = 0xd98e;
    in_struct_1.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    mixed_fn_1010_830a(ctx.g_struct_73_1050_14cc, 0x9b);
    in_struct_1.field_0xa = local_AX_18;
    in_struct_1.field_0xc = local_DX_49;
    return;
}

pub unsafe fn pass1_1008_d7da(param_1: &mut  u16) {
    let pu_var1: &mut  u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let mut i_var4: i32;
    let mut u_var5: u16;

  // u_var5 = (param_1  >> 0x10);
    i_var4 = param_1;
    unsafe { *param_1 = 0xd98e };
    (i_var4 + 2) = &ctx.PTR_LOOP_1050_1008;
    pu_var1 = (i_var4 + 10);
    u_var2 = (i_var4 + 0xc);
    if ((u_var2 | pu_var1) != 0) {
        unsafe { ppc_var3 = *pu_var1 };
        (**ppc_var3)();
    }
    pass1_1010_1d80(param_1);
    return;
}

pub unsafe fn pass1_1008_d968(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1008_d7da(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_d99e(param_1: &mut  Struct534, param_2: u16, param_3: u16) {
    pass1_1018_4cda(param_1, CONCAT22(param_3, param_2));
    CONCAT22(param_2, param_1) = 0xd9fa;
    param_1.u16_x02 = &ctx.PTR_LOOP_1050_1008;
    pass1_1018_4dce(CONCAT22(param_2, param_1), 0x9a);
  ctx._PTR_LOOP_1050_4230 = CONCAT22(param_2, param_1);
    return;
}

pub unsafe fn pass1_1008_d9d4(param_1: u32, param_2: u8) {
    win_cleanup_1018_4d22(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_dc2c(param_1: &mut  u16) {
    let mut local_es_4: u16;
    let mut temp_5f04f790ee: u32;

  // local_es_4 = (param_1  >> 0x10);
    unsafe { *param_1 = 0xdc80 };
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    error_check_1000_17ce((param_1 + 0x18));
    pass1_1010_1d80(param_1);
    return;
}

pub unsafe fn pass1_1008_dc5a(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1008_dc2c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_dc90(in_struct_1: &mut  Struct301, param_2: u32, param_3: u32) {
    let local_bx_4: &mut  Struct301;
    let mut u_var1: u16;

  // u_var1 = (in_struct_1  >> 0x10);
    local_bx_4 = in_struct_1;
    in_struct_1 = ctx.s_1_1050_389a;
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    local_bx_4.field_0x4 = param_3;
    local_bx_4.field_0x8 = param_2;
    local_bx_4.field_0xc = 0;
    local_bx_4.field_0xe = 0;
    local_bx_4.field_0x12 = 0;
    in_struct_1 = 0xdd4a;
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub unsafe fn pass1_1008_dd1e(param_1: &mut  u16, param_2: u8) {
    unsafe { *param_1 = ctx.s_1_1050_389a };
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_14f0_01b8(param_1: u8, param_2: i32) {
    let mut cVar1: u8;
    let mut in_CL: u8;
    let local_SP: &mut  u16;
    let local_BP__1: &mut  u16;
    let unaff_si: String;

    let mut local_DS__1: u16;
    let string_1: String;

    local_SP = &stack0xfffe;
    cVar1 = '\x1e';
    while {
        local_BP__1 = local_BP__1 + -1;
        local_SP = local_SP + -1;
        unsafe { *local_SP = *local_BP__1 };
        cVar1 = cVar1 + -1;
        '\0' < cVar1
    } {}
    string_1 = unaff_si + param_2;
    unsafe { *string_1 = *string_1 - in_CL };
    string_1 = unaff_si + param_2;
    unsafe { *string_1 = *string_1 + param_1 };
    string_1 = unaff_si + param_2;
    unsafe { *string_1 = *string_1 + param_1 };
    string_1 = unaff_si;
    unsafe { *string_1 = *string_1 + in_CL };
    return;
}

pub unsafe fn pass1_11a0_03fl4(param_1: String, param_2: u16, param_3: String) {
    let pi_var1: &mut  i32;
    let pu_var2: Vec<u8>;
    let pb_var3: Vec<u8>;
    let mut b_var4: u8;
    let mut b_var5: u8;
    let mut u_var6: i32;
    let mut cVar7: u8;
    let mut b_var8: u8;
    let string_2: String;
    let mut cVar9: u8;
    let string_5: String;
    let string_6: String;
    let local_BP_188: Vec<u8>;
    let string_4: String;
    let string_1: String;

    let mut local_DS__1: u16;
    let mut b_var10: bool;
    let in_stack_000003fa: String;
    let in_stack_000003fc: String;
    let mut in_stack_000003fe: u16;
    let mut in_stack_00000400: i32;
    let local_76ef: u8;
    let mut temp_13fb229f94a: u16;
    let mut temp_5f472f0e69: u16;
    let bytes_2: Vec<u8>;
    let string_3: String;

    bytes_2 = (param_3 + string_4);
    b_var4 = param_1;
    unsafe { *bytes_2 = *bytes_2 | b_var4 };
    string_3 = param_3;
    unsafe { *string_3 = *string_3 + 0x2 };
    string_3 = param_3 + string_4;
    unsafe { *string_3 = *string_3 + b_var4 };
    string_3 = param_3 + string_4;
    unsafe { *string_3 = *string_3 };
    local_BP_188 = &stack0x86fa;
    bytes_2 = (param_3 + string_4);
    unsafe { *bytes_2 = *bytes_2 | b_var4 };
    string_3 = param_3;
    unsafe { *string_3 = *string_3 + 0x2 };
    string_3 = param_3 + string_4;
    unsafe { *string_3 = *string_3 + b_var4 };
    string_3 = param_3 + string_4;
    unsafe { *string_3 = *string_3 };
    param_3[string_4] = -0x5e;
    string_3 = string_4;
    cVar9 = (param_2 >> 8);
    unsafe { *string_3 = *string_3 + cVar9 };
    string_3 = param_3 + string_4;
    cVar7 = param_2;
    unsafe { *string_3 = *string_3 + cVar7 };
    string_3 = (param_3 + string_1 + 0x217);
    unsafe { *string_3 = *string_3 + b_var4 };
    string_3 = param_3 + string_4;
    unsafe { *string_3 = *string_3 + b_var4 };
    string_3 = param_3 + string_4;
    unsafe { *string_3 = *string_3 };
    param_3[string_4] = -0x51;
    string_3 = string_4;
    unsafe { *string_3 = *string_3 + cVar9 };
    string_3 = param_3 + string_4;
    unsafe { *string_3 = *string_3 + cVar7 };
    pu_var2 = &local_76ef + string_1;
    unsafe { *pu_var2 = *pu_var2 + b_var4 };
    string_3 = param_3 + string_4;
    unsafe { *string_3 = *string_3 + b_var4 };
    string_3 = param_3 + string_4;
    unsafe { *string_3 = *string_3 };
    temp_5f472f0e69 = (param_3 + string_4);
    string_3 = param_3 + string_1;
    cVar9 = (param_3 >> 8);
    unsafe { *string_3 = *string_3 + cVar9 };
    string_3 = param_3 + string_4;
    unsafe { *string_3 = *string_3 + cVar7 };
    pi_var1 = (&stack0x86fa + string_4);
    unsafe { *pi_var1 = *pi_var1 + 1 };
    bytes_2 = &stack0x86fa + string_4;
    unsafe { b_var5 = *bytes_2 };
    unsafe { *bytes_2 = *bytes_2 + b_var4 };
    string_3 = &stack0x875b + string_4;
    unsafe { *string_3 = *string_3 + 'w' + CARRY1(b_var5, b_var4) };
    bytes_2 = (string_4 + 0x6f);
    unsafe { *bytes_2 = *bytes_2 & b_var4 };
    string_2 = param_1;
    string_5 = param_1;
    string_6 = (param_3 & 0xff | (cVar9 * 0x2) << 8);
    let bytes_2_val = unsafe { *bytes_2 };
    if (bytes_2_val != 0) {
        cVar7 = _in(param_1);
        unsafe { *string_1 = cVar7 };
        temp_13fb229f94a = _in(param_1);
        (string_1 + 1) = temp_13fb229f94a;
        local_BP_188 = ((param_1 + 0x69) * 0x676e);
        string_3 = in_stack_000003fa;
        unsafe { *string_3 = *string_3 + in_stack_00000400 };
        u_var6 = in_stack_00000400 & (in_stack_000003fa + param_1);
        b_var5 = u_var6 ^ in_stack_000003fa[param_1];
        string_2 = (u_var6 & 0xff00 | b_var5);
        bytes_2 = (in_stack_000003fa + param_1);
        unsafe { *bytes_2 = *bytes_2 | b_var5 };
        string_3 = param_1 + -0x3a00;
        unsafe { *string_3 = *string_3 + b_var5 };
        pu_var2 = local_BP_188 + param_1;
        unsafe { *pu_var2 = *pu_var2 + (in_stack_000003fc >> 8) };
        param_2 = in_stack_000003fe;
        string_5 = in_stack_000003fc;
        string_6 = in_stack_000003fa;
        string_4 = param_1;
        string_1 = param_1;
    }
    string_3 = (local_BP_188 + string_1 + 0x217);
    unsafe { *string_3 = *string_3 + (string_6 >> 8) };
    pu_var2 = local_BP_188 + string_4;
    b_var5 = string_2;
    unsafe { *pu_var2 = *pu_var2 + b_var5 };
    string_3 = string_6 + string_4;
    unsafe { *string_3 = *string_3 };
    string_6[string_4] = '2';
    string_3 = 0x100;
    cVar7 = param_2;
    unsafe { *string_3 = *string_3 + cVar7 };
    string_3 = string_6 + string_1;
    unsafe { *string_3 = *string_3 + b_var5 };
    string_3 = string_6 + string_1;
    unsafe { *string_3 = *string_3 + b_var5 };
    if (string_5 != 0xffff) {
        // WARNING: Bad instruction - Truncating control flow here
        halt_baddata();
    }
    if (string_5 == 0xffff) {
        // WARNING: Bad instruction - Truncating control flow here
        halt_baddata();
    }
    string_5 = string_4 + 1;
    let string_4_val = unsafe { *string_4 };
    out(string_4_val, 0);
    string_3 = string_6 + 0x6b;
    unsafe { *string_3 = *string_3 + cVar7 };
    string_3 = string_6 + string_5;
    unsafe { *string_3 = *string_3 + b_var5 };
    b_var8 = cVar7 - 1;
    pu_var2 = local_BP_188 + string_5;
    unsafe { *pu_var2 = *pu_var2 + b_var5 };
    bytes_2 = 0x3000;
    unsafe { b_var4 = *bytes_2 };
    unsafe { *bytes_2 = *bytes_2 + b_var8 };
    string_3 = string_6 + string_5;
    unsafe { *string_3 = (*string_3 - b_var5) - CARRY1(b_var4, b_var8) };
    string_3 = string_6 + (string_1 + -1);
    unsafe { *string_3 = *string_3 + b_var5 };
    b_var10 = string_2 * 0x100 == -1;
    if (b_var10) {
        if (!b_var10) {
            pb_var3 = (string_4 + 2);
            let string_5_val = unsafe { *string_5 };
            out(string_5_val, 0);
            pu_var2 = local_BP_188 + 0x65;
            unsafe { *pu_var2 = *pu_var2 + b_var8 };
            string_3 = string_6;
            b_var8 = (string_2 >> 8);
            unsafe { *string_3 = *string_3 + b_var8 };
            pu_var2 = local_BP_188 + pb_var3;
            unsafe { *pu_var2 = *pu_var2 };
            b_var5 = b_var5 ^ string_6[pb_var3];
            bytes_2 = pb_var3;
            unsafe { b_var4 = *bytes_2 };
            unsafe { *bytes_2 = *bytes_2 + b_var8 };
            string_3 = string_6 + pb_var3;
            unsafe { *string_3 = (*string_3 - b_var5) - CARRY1(b_var4, b_var8) };
            string_3 = string_6 + (string_1 + -2);
            unsafe { *string_3 = *string_3 + b_var5 };
            // WARNING: Bad instruction - Truncating control flow here
            halt_baddata();
        }
        // WARNING: Bad instruction - Truncating control flow here
        halt_baddata();
    }
    // WARNING: Bad instruction - Truncating control flow here
    halt_
}

pub unsafe fn pass1_1050_3552() {
    let mut cVar1: u8;
    let p_uvar2: &mut  u16;
    let unaff_bp: &mut  u16;
    let mut unaff_ss: u16;
    let mut unaff_DS: u16;

    pu_var2 = &stack0xfffe;
    cVar1 = '\x16';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var2 = pu_var2 + -1;
        unsafe { *pu_var2 = *unaff_bp };
        cVar1 = cVar1 + -1;
        '\0' < cVar1
    } {}
    return;
}

pub unsafe fn pass1_1050_3654() {
    let mut cVar1: u8;
    let p_uvar2: &mut  u16;
    let unaff_bp: &mut  u16;
    let mut unaff_ss: u16;
    let mut unaff_DS: u16;

    pu_var2 = &stack0xfffe;
    cVar1 = '\x16';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var2 = pu_var2 + -1;
        unsafe { *pu_var2 = *unaff_bp };
        cVar1 = cVar1 + -1;
        '\0' < cVar1
    } {}
    return;
}

pub unsafe fn pass1_1050_37d4() {
    let mut cVar1: u8;
    let p_uvar2: &mut  u16;
    let unaff_bp: &mut  u16;
    let mut unaff_ss: u16;
    let mut unaff_DS: u16;

    pu_var2 = &stack0xfffe;
    cVar1 = '\x16';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var2 = pu_var2 + -1;
        unsafe { *pu_var2 = *unaff_bp };
        cVar1 = cVar1 + -1;
        '\0' < cVar1
    } {}
    return;
}
