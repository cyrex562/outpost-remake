use crate::pass::pass17_funcs::{pass1_1030_73a8, pass1_1030_70ac, pass1_1030_7064, pass1_1030_70f4, pass1_1030_7c28, pass1_1030_6ddc, pass1_1030_7f98, pass1_1030_7f5a};
use crate::pass::pass12_funcs::pass1_1008_c6ae;
use crate::structs::prog_structs_16::Struct493;
use crate::app_context::AppContext;
use crate::structs::prog_structs_2::{Struct492, Struct199, Struct7};
use crate::pass::pass8_funcs::pass1_1010_1d80;
use crate::util::{CONCAT22, CARRY2, CONCAT11};
use crate::err_ops::error_check_1000_17ce;
use crate::pass::pass20_funcs::{pass1_1018_04f2, pass1_1010_a478};
use crate::structs::prog_structs_7::{Struct376, Struct44};
use crate::typedefs::HDC16;
use crate::pass::pass14_funcs::pass1_fn_1008_612e;
use crate::pass::pass_funcs::pass1_1000_4906;
use crate::pass::pass19_funcs::pass1_1018_dcf6;
use crate::struct_ops::struct_ops_2::{process_struct_1000_179c, process_struct_1010_1d48, process_struct_1010_20ba};
use crate::structs::prog_structs_30::Struct496;
use crate::structs::prog_structs_29::Struct375;
use crate::pass::pass6_funcs::pass1_1038_4d28;
use crate::pass::pass4_funcs::{pass1_1028_b58e, pass1_1028_e1ec};
use crate::pass::pass3_funcs::pass1_1028_121e;
use crate::pass::pass16_funcs::pass1_1028_4faa;
use crate::structs::prog_structs_26::Struct494;
use crate::string_ops::misc::{string_fn_1000_3f9c, copy_string_1000_3d3e, big_switch_statement_1020_c222};
use crate::string_ops::load::load_string_1010_84e0;
use crate::structs::prog_structs_31::Struct491;
use crate::mem_funcs::mem_ops_1::get_fn_ptr_at_address;
use crate::structs::prog_structs_24::Struct2111;

pub unsafe fn pass1_1010_e1f4(ctx: &mut AppContext, param_1: u32, param_2: &mut Struct493) {
    let mut u_var1: i32;
    let mut c_var2: u8;
    let b_var3: bool;
    let pu_var4: Vec<u8>;
    let mut u_var5: u16;
    
    
    // let mut i32_var6: i32;
    let mut u_var7: u32;
    let mut u_var8: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    // i32_var6 = param_1;
    *(param_1 + 0x13c) = 0;
    u_var7 = pass1_1030_73a8(ctx, param_2);
    //// _var5 = (u_var7  >> 0x10);
    u_var1 = (u_var7 + 0xc);
    b_var3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var1, 0xc);
    if (b_var3 != false)
        || (
        b_var3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var1, 0x14),
        b_var3 != false,
        )
    {
        u_var8 = 0x5d7;
        // goto LAB_1010_e241;
    }
    b_var3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var1, 10);
    if (b_var3 != 0) {
        u_var8 = 0x657;
        // goto LAB_1010_e241;
    }
    b_var3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var1, 0x15);
    if (b_var3 != 0) {
        u_var8 = 0x4f8;
        // goto LAB_1010_e241;
    }
    b_var3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var1, 0xb);
    if (b_var3 != 0) {
        u_var8 = 0x658;
        // goto LAB_1010_e241;
    }
    b_var3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var1, 0x16);
    if (b_var3 != 0) {
        u_var8 = 0x659;
        // goto LAB_1010_e241;
    }
    b_var3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var1, 0x17);
    if (b_var3 != 0) {
        u_var8 = 0x65a;
        // goto LAB_1010_e241;
    }
    b_var3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var1, 0x21);
    if (b_var3 != 0) {
        u_var8 = 0x65b;
        // goto LAB_1010_e241;
    }
    b_var3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var1, 0x1c);
    if (b_var3 != 0) {
        u_var8 = 0x65c;
        // goto LAB_1010_e241;
    }
    b_var3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var1, 0x1d);
    if (b_var3 != 0) {
        u_var8 = 0x65d;
        // goto LAB_1010_e241;
    }
    b_var3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var1, 0x18);
    if (b_var3 != 0) {
        u_var8 = 0x65e;
        // goto LAB_1010_e241;
    }
    b_var3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var1, 0x19);
    if (b_var3 != 0) {
        u_var8 = 0x65f;
        // goto LAB_1010_e241;
    }
    b_var3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var1, 4);
    if ((b_var3 != 0)
        || (
        b_var3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var1, 3),
        b_var3 != 0,
        ))
    {
        u_var8 = 0x660;
        // goto LAB_1010_e241;
    }
    b_var3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var1, 0x1e);
    if (b_var3 != 0) {
        u_var8 = 0x662;
        // goto LAB_1010_e241;
    }
    b_var3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var1, 0x23);
    if ((b_var3 != 0)
        || (
        b_var3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var1, 0x1b),
        b_var3 != 0,
        ))
    {
        // LAB_1010_e3c2:
        u_var8 = 0x663;
        // goto LAB_1010_e241;
    }
    b_var3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var1, 0x1f);
    if (b_var3 != 0) {
        u_var8 = 0x42e;
        // goto LAB_1010_e241;
    }
    b_var3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var1, 1);
    if (b_var3 != 0) {
        u_var8 = 0x5df;
        // goto LAB_1010_e241;
    }
    b_var3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var1, 2);
    if ((b_var3 != 0)
        || (
        b_var3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var1, 0x13),
        b_var3 != 0,
        ))
    {
        u_var8 = 0x42f;
        // goto LAB_1010_e241;
    }
    b_var3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var1, 0x20);
    if (b_var3 != 0) {
        u_var8 = 0x632;
        // goto LAB_1010_e241;
    }
    b_var3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var1, 0xe);
    if (b_var3 != 0) {
        u_var8 = 0x5e5;
        // goto LAB_1010_e241;
    }
    b_var3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var1, 0x10);
    if (b_var3 != 0) {
        u_var8 = 0x5d4;
        // goto LAB_1010_e241;
    }
    b_var3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var1, 0x12);
    if ((b_var3 != 0)
        || (
        b_var3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var1, 0x11),
        b_var3 != 0,
        ))
    {
        pass1_1010_e58a(param_1, u_var7);
        u_var5 = ctx.dx_reg;
        // LAB_1010_e4a9:
        copy_string_1000_3d3e(
            param_1 + 0x13c,
            CONCAT22(u_var5, b_var3),
        );
        return;
    }
    pu_var4 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var1, 6);
    if (pu_var4 != 0x0) {
        pass1_1030_7f5a(param_2);
        u_var5 = ctx.dx_reg;
        b_var3 = big_switch_statement_1020_c2f8(pu_var4);
        // goto LAB_1010_e4a9;
    }
    b_var3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var1, 5);
    if (b_var3 != 0) {
        u_var8 = pass1_1030_7f98(param_2);
        b_var3 = big_switch_statement_1020_c222(u_var8);
        // goto LAB_1010_e4a9;
    }
    b_var3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var1, 0x40);
    if (b_var3 != 0) {}
    // goto LAB_1010_e3c2;
    if (u_var1 == 0x6a) {
        u_var8 = 0x531;
        // goto LAB_1010_e241;
    }
    if (u_var1 < 0x6b) {
        c_var2 = u_var1;
        if (c_var2 == '!') {
            u_var8 = 0x5e4;
            // goto LAB_1010_e241;
        }
        if (c_var2 < '\"') {
            if (c_var2 == '\n') {
                u_var8 = 0x654;
                // goto LAB_1010_e241;
            }
            if (c_var2 == '\x0f') {
                // LAB_1010_e54c:
                u_var8 = 0x655;
                // goto LAB_1010_e241;
            }
            if (c_var2 == '\x11') {
                u_var8 = 0x656;
                // goto LAB_1010_e241;
            }
        } else {
            if (c_var2 == ',') {
                u_var8 = 0x69b;
                // goto LAB_1010_e241;
            }
            if (c_var2 == '@') {
                u_var8 = 0x600;
                // goto LAB_1010_e241;
            }
            if (c_var2 == 'd') {}
            // goto LAB_1010_e54c;
        }
    }
    u_var8 = 0x5ea;
    // LAB_1010_e241:
    load_string_1010_84e0(
        ctx,
        0x3ff,
        param_1 + 0x13c,
        u_var8
    );
    return;
}

// WARNING: Variable defined which should be unmapped: local_12
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_e58a(param_1: u32, param_2: u32) {
    let mut iVar1: i32;
    let ppc_var2: fn();
    let mut u_var3: i32;
    let b_var4: bool;
    let pu_var5: &mut  u32;
    let mut u_var6: i32;
    let mut i_var7: i32;
    let mut u_var8: u16;
    let pp_var9: Struct2111;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    i_var7 = param_1;
    *(i_var7 + 0x13c) = 0;
  // u_var8 = (param_2  >> 0x10);
    pu_var5 = (param_2 + 0x20);
    iVar1 = (param_2 + 0xc);
    b_var4 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, iVar1, 0x11);
    if (b_var4 == 0) {
        b_var4 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, iVar1, 0x12);
        if (b_var4 == 0) {
            return;
        }
        pp_var9 = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(local_12, 0x31));
        ppc_var2 = (pp_var9 + 0x14);
        ppc_var2(
            &ctx.PTR_LOOP_1050_1008,
            pp_var9,
            (pp_var9 >> 0x10),
            pu_var5,
            pu_var5 >> 0xf,
        );
        u_var6 = ctx.dx_reg | pu_var5;
        u_var3 = ctx.dx_reg;
    } else {
        pp_var9 = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(local_12, 0x41));
        ppc_var2 = (pp_var9 + 0x14);
        ppc_var2(
            &ctx.PTR_LOOP_1050_1008,
            pp_var9,
            (pp_var9 >> 0x10),
            pu_var5,
            pu_var5 >> 0xf,
        );
        u_var6 = ctx.dx_reg | pu_var5;
        u_var3 = ctx.dx_reg;
    }
    if (u_var6 == 0) {
        load_string_1010_84e0(
            ctx,
            0x3ff,
            i_var7 + 0x13c,
            0x531,
        );
    } else {
        unsafe { ppc_var2 = (*pu_var5 + 0x14) };
        ppc_var2(&ctx.PTR_LOOP_1050_1008, pu_var5, u_var3);
        copy_string_1000_3d3e(
            i_var7 + 0x13c,
            CONCAT22(ctx.dx_reg, pu_var5),
        );
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_e682(ctx: &mut AppContext, param_1: &mut  Struct491, param_2: &mut  Struct493) {
    let mut var1: u32;
    let mut var2: String;
    let mut var3: bool;
    let mut var4: Struct493;
    let mut var5: u16 = 0;
    let var9: &mut  Struct491;
    let var6: String;
    let mut var10: u16;
    let var7: u8;
    let var8: String;
    let mut var11: String;
    let mut var12: u16;
    let mut var13: String;
    let mut var14: u16;
    let mut var15: String;
    let mut var16: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: i32;
    let mut local_6: u16;
    let mut local_4: u16;

  // pcVar6 = (param_1  >> 0x10);
  //   local_bx_4 = param_1;
    param_1.field_0x13c = 0;
    _local_6 = pass1_1030_73a8(ctx,param_2);
  // u_var5 = (_local_6  >> 0x10);
    local_8 = (_local_6 + 0xc);
    var2 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, local_8, 1);
    if ((var2 == 0x0)
        && (
        var2 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, local_8, 0x13),
        var2 == 0x0,
        ))
        && (
        var2 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, local_8, 2),
        var2 == 0x0,
        )
    {
        var3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, local_8, 0xe);
        if var3 != 0 {
            var1 = param_1.field_0x138;
            _local_10 = (var1 + 0x24);
          // struct_var4 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, _local_10, (_local_10  >> 0x10));
            _local_c = var4;
            var16 = var4[0x10].field_0x16;
            var2 = *(var16 + 0x1a8);
            var8 = ctx.s__u_1050_3947.clone();
            var15 = var2;
            // LAB_1010_e76d:
            string_fn_1000_3f9c(
                ctx,
                &param_1.field_0x13c,
                &var6,
                &var8,
                &ctx.g_alloc_addr_1050_1050,
                var2,
            );
            return;
        }
        var3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, local_8, 5);
        if (var3 == false)
            && (
            var3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, local_8, 6),
            var3 == false,
            )
        {
            var2 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, local_8, 0x10);
            if (var2 == 0x0) {
                var2 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, local_8, 0xc);
                if (var2 == 0x0)
                    && (
                    var2 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, local_8, 0x14),
                    var2 == 0x0,
                    )
                {
                    var2 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, local_8, 10);
                    if var2 == 0x0 {
                        var2 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, local_8, 0x1e);
                        if var2 == 0x0 {
                            load_string_1010_84e0(
                                ctx,
                                0x3ff,
                                &mut param_1.field_0x13c,
                                0x598,
                            );
                            return;
                        }
                        pass1_1030_6ddc(param_2);
                        var8 = s__u_1050_395c;
                        var11 = var2;
                        // goto LAB_1010_e76d;
                    }
                    pass1_1030_7c28(&param_2.field_0x0, 0x21);
                    var7 = 0x58;
                    var11 = var2.clone();
                    var12 = var5;
                } else {
                    pass1_1010_e8f6(ctx, param_1, &var6, param_2);
                    var14 = ctx.dx_reg;
                    var11 = var2;
                    var12 = ctx.dx_reg;
                    pass1_1030_7c28(&var2, 0x23);
                    var7 = 0x54;
                    var13 = var2;
                }
            } else {
                pass1_1030_7c28(&param_2.field_0x0, 0x1e);
                var7 = 0x50;
                var11 = var2;
                var12 = var5;
            }
        } else {
            var11 = 0;
            var13 = 0;
            pass1_1010_e8d0(
                param_1,
                &var6,
                &var13,
                &var11,
                param_2,
            );
            var7 = 0x4a;
            var2 = var11;
        }
    } else {
        pass1_1010_e8f6(ctx, param_1, var6, param_2);
        _local_c = CONCAT22(ctx.dx_reg, var2);
        pass1_1030_70f4(CONCAT22(ctx.dx_reg, var2));
        _local_10 = CONCAT22(ctx.dx_reg, var2);
        var7 = 0x43;
    }
    // CONCAT11(0x39, u_var7),
    string_fn_1000_3f9c(
        ctx, &param_1.field_0x13c,
        &var6,
        &ctx.g_alloc_addr_1050_1050,
        &var2,
    );
    return;
}

pub unsafe fn pass1_1010_e8d0(
    param_1: &mut Struct491,
    param_2: &String,
    param_3: &String,
    param_4: &String,
    param_5: &mut Struct493,
) -> i32 {
    let mut u_var1: u16;
    u_var1 = pass1_1030_7064(param_5);
    *param_4 = u_var1;
    u_var1 = pass1_1030_70ac(param_5);
    *param_3 = u_var1;
    0
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_e8f6(ctx: &mut AppContext, param_1: &mut Struct491, param_2: &String, param_3: &mut Struct493) {
    let mut iVar1: i32;
    let b_var2: bool;
    let local_struct_1: &mut  Struct44;
    
    let mut u_var3: u16;
    
    let mut u_var4: Struct44;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    u_var4 = pass1_1030_73a8(ctx,param_3).unwrap();
    iVar1 = (u_var4 + 0xc);
    b_var2 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, iVar1, 0x13);
    if b_var2 == false {
        b_var2 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, iVar1, 0x14);
        if b_var2 == false {
            return;
        }
        pass1_1028_4faa(ctx, &mut u_var4);
        local_struct_1 = &mut u_var4;
        u_var3 = ctx.dx_reg;
    } else {
        pass1_1028_121e(&mut u_var4);
        local_struct_1 = &mut u_var4;
        u_var3 = ctx.dx_reg;
    }
    pass1_1028_b58e(local_struct_1);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_e964() {
    let mut u_var1: u32;
    let paVar2: &mut  Struct493;
    let mut u_var3: u16;
    let ppVar4: &mut  Struct2551;
    let mut in_stack_0000ffea: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar4 = process_struct_1010_20ba(
        ctx.g_struct_var_1050_0ed0,
        CONCAT22(in_stack_0000ffea, 0x2f),
    );
  // u_var3 = (ppVar4  >> 0x10);
    u_var1 = (ppVar4 + 0x24);
  paVar2 = pass1_1028_e1ec(ctx, ctx._PTR_LOOP_1050_65e2, u_var1, );
    pass1_1038_4d28(paVar2);
    return;
}

pub unsafe fn pass1_1010_e9a6(ctx: &mut AppContext, param_1: &mut Struct7, param_2: u8) -> &mut Struct7 {
    pass1_1010_a478(ctx,param_1);
    if (param_2 & 1) != 0 {
        error_check_1000_17ce(ctx, param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_e9e4(ctx: &mut AppContext, param_1: &mut  Struct375, param_3: u16) {
    let pu_var1: &mut  u32;
    let mut u_var2: i32;
    let mut i_var3: i32;
    let mut u_var4: i32;
    let mut u_var5: i32;
    let mut u_var6: u32;
    let struct_a: Struct7 = Struct7::new();
    let pa_var7: &mut  Struct7;
    let mut local_dx_149: u16;
    let local_si_301: &mut  Struct496;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_4: u16;

    process_struct_1010_1d48(param_1, param_3);
    param_1.ptr_2_lo = ctx.s_1_1050_389a;
    param_1.ptr_2_hi = ctx.PTR_LOOP_1050_1008;
    param_1.ptr_2_lo = ctx.s_18_2_1050_3aa5 + 3;
    param_1.ptr_2_hi = ctx.PTR_LOOP_1050_1008;
    u_var5 = 0;
    param_1.u32_x0e = 0;
    param_1.u32_x12 = 0;
    param_1.u16_x16 = 0;
    param_1.field_0x1a = 0;
    param_1.field_0x1e = 0;
    param_1.field_0x20 = 0;
    param_1.field_0x24 = 0;
    param_1.field_0x28 = 0;
    param_1.field_0x2c = 0;
    param_1.field_0x30 = 0;
    param_1.field_0x32 = 0;
    param_1.ptr_1_lo = 0x558;
    param_1.ptr_1_hi = 0x1018;
    param_1.ptr_2_lo = 0x568;
    param_1.ptr_2_hi = 0x1018;
    pa_var7 = struct_a;
    process_struct_1000_179c(ctx,4, struct_a);
    if (pa_var7 | u_var5) == 0 {
        &param_1.u32_x0e = 0;
    } else {
        pass1_1018_dcf6(pa_var7);
        param_1.u32_x0e = u_var5;
        param_1.field_0x10 = local_dx_149;
    }
    pass1_1000_4906(&mut param_1.field_0x34, 0, 0x24);
    param_1.field_0x38 = 0xfa;
    param_1.field_0x3c = 0x15e;
    u_var6 = 0x1c2;
    param_1.field_0x40 = 0x1c2;
    param_1.field_0x44 = 0x1c2;
    param_1.field_0x46 = 0x2260000;
    param_1.field_0x4a = 0x28a0000;
    param_1.field_0x4e = 0x730000;
    param_1.field_0x52 = 0x960000;
    param_1.field_0x56 = 0;
    local_4 = 1;
    while local_4 < 9 {
        pass1_fn_1008_612e(0, 0x1d);
        u_var5 = u_var6;
        pass1_fn_1008_612e(1, 2);
        if (u_var6 & 1) != 0 {
            u_var5 = -u_var5;
        }
        local_si_301 = (local_4 * 4);
        pu_var1 = (local_si_301 + &param_1.field_0x34);
        unsafe {
            u_var2 = *pu_var1;
            u_var4 = u_var5 + *pu_var1;
        }
        u_var6 = u_var4;
        i_var3 = (local_si_301 + &param_1.field_0x34 + 2);
        (param_1 + local_si_301 + 0x34) = u_var4;
        (param_1 + local_si_301 + 0x36) = (u_var5 >> 0xf) + i_var3 + CARRY2(u_var5, u_var2);
        local_4 = local_4 + 1;
    }
    return;
}

pub unsafe fn pass1_1010_eb66(ctx: &mut AppContext, param_1: &mut  Struct7) {
    let pu_var1: &mut  u32;
    let mut u_var2: i32;
    let var3: &mut  HDC16;
    // let local_struct_1: &mut  Struct376;
    let mut var4: u16;
    let mut local_e: String;
    let temp_8628aef2705: Vec<u8>;
    let fn_ptr_1: fn();

  // u_var4 = (in_struct_1  >> 0x10);
  //   local_struct_1 = param_1;
    param_1.ptr_a_lo = 0x558;
    param_1.ptr_a_hi = 0x1018;
    param_1.dc_handle_x0a = 0x568;
    param_1.u16_x0c = 0x1018;
    pu_var1 = param_1.u16_x0e;
    u_var2 = param_1.u16_x10;
    if ((u_var2 | pu_var1) != 0) {
        // unsafe {
        //     fn_ptr_1 = *pu_var1;
        //     (**fn_ptr_1)();
        // }
        get_fn_ptr_at_address(*pu_var1)();
    }
    pass1_1018_04f2(param_1);
    error_check_1000_17ce(ctx, &mut param_1.u32_x2c);
    if (param_1 == 0x0) {
        var3 = 0x0;
        var4 = 0;
    } else {
        var3 = &mut param_1.dc_handle_x0a;
    }
    local_e = CONCAT22(var4, var3);
    local_e = ctx.s_1_1050_389a.clone();
    var3[1] = ctx.PTR_LOOP_1050_1008;
    pass1_1010_1d80(param_1);
    return;
}

pub unsafe fn pass1_1010_ebf8(param_1: u32, param_2: u16, param_3: u16, param_4: u16) {
    let mut local_es_16: u16;

  // local_es_16 = (param_1  >> 0x10);
    (param_1 + param_4 * 4 + 0x34) = param_2;
    (param_1 + param_4 * 4 + 0x36) = param_3;
    return;
}
