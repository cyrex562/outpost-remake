use crate::app_context::AppContext;
use crate::draw::palette;
use crate::pass::pass14_funcs::{pass1_1008_43cc, pass1_1008_4d84};
use crate::pass::pass8_funcs::process_struct_1010_20ba;
use crate::prog_structs::prog_structs_16::Struct115;
use crate::prog_structs::prog_structs_2::Struct199;
use crate::prog_structs::prog_structs_6::{Struct622, Struct670};
use crate::struct_ops::process_struct_1008_4772;
use crate::sys_ops::get_sys_metrics_1020_7c1a;
use crate::typedefs::HWND16;
use crate::util::{CONCAT12, CONCAT13, CONCAT22, SUB41};
use crate::winapi::{CreateDC16, GetDC16};

pub fn get_gui_dc_1018_4db0(in_struct_1: Vec<u8>, in_win_handle: HWND16) -> *mut Struct199 {
    let dev_ctx_handle: *mut Struct199;
    let local_struct_1_hi: Vec<u8>;
    local_struct_1_hi = (in_struct_1 >> 0x10);
    (in_struct_1 + 0x16) = in_win_handle;
    dev_ctx_handle = GetDC16(in_win_handle);
    (in_struct_1 + 0x14) = dev_ctx_handle;
    return dev_ctx_handle;
}

pub unsafe fn create_drawing_dc_1018_4e04(
    ctx: &mut AppContext,
    param_1: *mut Struct115,
    param_2: u16,
    param_3: u16,
    param_4: u16,
) {
    let pp_var1: fn();
    let mut dc_handle: u16;
    let mut hdc_ptr: u16;
    let local_bx_8: *mut Struct115;
    let mut u_var2: u16;
    let mut u_var3: u32;
    let mut local_10: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut func_ptr: u32;
    let mut inner_struct: u32;
    let mut offset: u16;

    u_var2 = (param_1 >> 0x10);
    local_bx_8 = param_1;
    pp_var1 = (param_1 + 0x14);
    (**pp_var1)();
    u_var3 = process_struct_1008_4772(local_bx_8.field_0xa);
    pass1_1008_43cc(local_bx_8.field_0xa);
    dc_handle = CreateDC16(u_var3, (u_var3 >> 0x10), 0x0, 0x42340000);
    local_bx_8.dc_handle = dc_handle;
    hdc_ptr = &local_bx_8.dc_handle;
    inner_struct = local_bx_8.field_0xa;
    func_ptr = (local_bx_8.field_0xa + 8);
    (**func_ptr)(
        offset,
        inner_struct,
        (inner_struct >> 0x10),
        hdc_ptr,
        u_var2,
        param_1,
        param_2,
    );
    local_bx_8.field_0x1a = hdc_ptr;
    if (ctx.WORD_1050_422e != 0) && (0x280 < param_4) {
        local_10 = 0;
        while local_10 < (ctx.u16_1050_4216 + 1) {
            (&ctx.PTR_BYTE_1050_0009_1050_4172 + local_10 * 2) =
                (((&ctx.PTR_BYTE_1050_0009_1050_4172 + local_10 * 2) * (param_4 + 1)) / 0x280);
            local_10 = local_10 + 1;
        }
        local_10 = 0;
        while local_10 < (ctx.WORD_1050_422c + 1) {
            (&ctx.WORD_1050_419a + local_10 * 2) =
                (((&ctx.WORD_1050_419a + local_10 * 2) * (param_3 + 1)) / 0x1e0);
            local_10 = local_10 + 1;
        }
    }
    ctx.WORD_1050_422e = 0;
    return;
}

pub unsafe fn get_dc_1020_921c(ctx: &mut AppContext, param_1: *mut Struct622, param_2: u16) {
    let local_AX_99: *mut Struct670;
    let mut u_var1: u16;
    let local_struct_1: *mut Struct622;
    let local_es_4: *mut Struct622;
    let ppVar2: *mut pass1_struct_1;
    let mut in_stack_0000ffe2: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var1 = (in_stack_0000ffe2 >> 0x10);
    local_es_4 = (param_1 >> 0x10);
    local_struct_1 = param_1;
    param_1.u16_0x0 = ctx.s_1_1050_389a;
    local_struct_1.u16_0x2 = &ctx.PTR_LOOP_1050_1008;
    param_1.u16_0x0 = (ctx.ctx.s_18_2_1050_3aa5 + 3);
    local_struct_1.u16_0x2 = &ctx.PTR_LOOP_1050_1008;
    local_struct_1.window_handle_0x4 = param_2;
    param_1.u16_0x0 = ctx.s_0_020_1050_3ab0;
    local_struct_1.u16_0x2 = &ctx.PTR_LOOP_1050_1008;
    &local_struct_1.u16_0x6 = 0;
    local_struct_1.field_0xa = 0;
    local_struct_1.field_0xc = 0;
    local_struct_1.field_0xe = 0;
    local_struct_1.field_0x10 = 0;
    local_struct_1.i16_0x12 = 0;
    param_1.u16_0x0 = 0x96c8;
    local_struct_1.u16_0x2 = 0x1020;
    local_AX_99 = GetDC16(local_struct_1.window_handle_0x4);
    local_struct_1.field_0xa = local_AX_99;
    ppVar2 = process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, CONCAT22(u_var1, 0x48));
    u_var1 = (ppVar2 >> 0x10);
    local_struct_1.field_0xc = (ppVar2 + 10);
    local_struct_1.field_0xe = (ppVar2 + 0xc);
    return;
}

pub unsafe fn get_dc_1020_717e(ctx: &mut AppContext, param_1: *mut u16, param_2: u32) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let pu_var3: *mut u32;
    let pu_var4: *mut u16;
    let pu_var5: *mut u32;

    let mut i32_var6: i32;
    let mut u_var7: u16;
    let unaff_ss: u8;
    let pp_var8: *mut pass1_struct_1;
    let u_var9: u8;
    let u_var10: u8;
    let mut i_var11: i32;
    let mut u_var12: u16;
    let mut in_stack_0000ffdc: u16;
    let mut local_1c: u16;
    let mut local_14: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let mut local_4: u16;

    i_var11 = param_1;
    u_var12 = (param_1 >> 0x10);
    get_sys_metrics_1020_7c1a(i_var11, u_var12, param_2, (param_2 >> 0x10));
    (i_var11 + 0x14) = 0;
    (i_var11 + 0x18) = param_2;
    (i_var11 + 0x1c) = 0;
    (i_var11 + 0x20) = 0;

    *param_1 = 0x754c;

    (i_var11 + 2) = 0x1020;
    pp_var8 =
        process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffdc, 4));
    u_var7 = (pp_var8 >> 0x10);
    (i_var11 + 0x1c) = pp_var8;
    (i_var11 + 0x1e) = u_var7;
    pp_var1 = ((i_var11 + 0x1c) + 4);
    (**pp_var1)(0x10, (i_var11 + 0x1c), u_var7, 0, i_var11, u_var12);
    local_4 = GetDC16((i_var11 + 4));
    u_var2 = (i_var11 + 0x1c);
    (u_var2 + 0x178) = local_4;
    u_var2 = (i_var11 + 0x1c);
    u_var7 = (u_var2 >> 0x10);
    i32_var6 = u_var2;
    pu_var5 = (i32_var6 + 0x24);
    u_var9 = SUB41(pu_var5, 0);
    u_var10 = (pu_var5 >> 8);
    pp_var1 = (*pu_var5 + 0x14);
    (**pp_var1)(0x38, u_var9, (i32_var6 + 0x26));
    pp_var8 = process_struct_1010_20ba(
        ctx._g_astruct_372_1050_0ed0,
        CONCAT13(u_var10, CONCAT12(u_var9, 0x29)),
    );
    u_var2 = (pp_var8 + 0xe);
    pu_var3 = u_var2;
    pass1_1008_4d84((pu_var5 & 0xffff | ctx.dx_reg << 0x10), pu_var3);
    pu_var4 = &local_4;
    palette::realize_palette_1008_4e08(pu_var3, u_var2, pu_var4, ctx.stack_seg_reg);
    (i_var11 + 0x20) = pu_var4;
}
