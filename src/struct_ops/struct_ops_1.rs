use crate::app_context::AppContext;
use crate::err_ops::error_check_1000_17ce;
use crate::mem_funcs::free_mem::free_mem_1000_093a;
use crate::structs::my_structs::Pass110309e9cInput;
use crate::structs::prog_structs_25::Struct82;
use crate::structs::prog_structs_2::Struct7;
use crate::structs::prog_structs_31::Struct125;

pub unsafe fn set_struct_fields_1008_7e98(ctx: &mut AppContext, param_1: &mut Struct7, param_2: u8) {
    // param_1.field_0 = 0x380a;
    param_1.field_0 = ctx.s_1_1050_389a.clone();
    param_1.field_2 = ctx.PTR_LOOP_1050_1008 as u32;

    if (param_2 & 1) != 0 {
        error_check_1000_17ce(ctx, param_1);
    }
}

pub fn struct_func_1000_0368(ctx: &mut AppContext) {
    let struct_var1: Struct125;

    if ctx.ax_reg.field_0x4 == ctx.ax_reg {
        (ctx.bx_reg + ctx.dx_reg * 2) = 0;
    } else {
        (ctx.ax_reg.field_0x6 + 4) = ctx.ax_reg.field_0x4;
        (ctx.ax_reg.field_0x4 + 6) = ctx.ax_reg.field_0x6;
        struct_var1 = (in_dx * 2 + in_bx);
        if struct_var1 == ctx.ax_reg {
            struct_var1 = ctx.ax_reg.field_0x4;
        }
    }
    ctx.ax_reg.field_0x4 = (ctx.bx_reg + 10);
    (ctx.bx_reg + 10) = ctx.ax_reg;
    return;
}

pub fn set_struct_1000_05b4(ctx: &mut AppContext, param_1: &mut Stuct82, param_2: u8) {
    param_1.field_0xa = 1;
    param_1.field_0x8 = 0x668;
    param_1.field_0x13 = -((param_2 & 2) != 0) & 2;
    param_1.field_0x10 = 0;
    param_1.field_0xe = 0;
    return;
}

pub unsafe fn set_struct_field_1030_9e9c(ctx: &mut AppContext, param_1: &mut Pass110309e9cInput, param_2: u8) {
    param_1.field_0x0 = ctx.s_1_1050_389a.clone();
    // (param_1 + 2) = ctx.PTR_LOOP_1050_1008;
    param_1.field_0x2 = ctx.PTR_LOOP_1050_1008;
    if (param_2 & 1) != 0 {
        free_mem_1000_093a(ctx, param_1);
    }
}
