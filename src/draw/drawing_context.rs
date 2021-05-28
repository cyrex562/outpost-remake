use crate::app_context::AppContext;
use crate::draw::palette;
use crate::pass::pass14_funcs::{pass1_1008_43cc, pass1_1008_4d84};
use crate::pass::pass8_funcs::process_struct_1010_20ba;
use crate::structs::prog_structs_16::Struct115;
use crate::structs::prog_structs_2::Struct199;
use crate::structs::prog_structs_6::{Struct622, Struct670};
use crate::struct_ops::struct_ops_2::process_struct_1008_4772;
use crate::sys_ops::get_sys_metrics_1020_7c1a;
use crate::typedefs::{HWND16, HDC16};
use crate::util::{CONCAT12, CONCAT13, CONCAT22, SUB41};
use crate::winapi::{CreateDC16, GetDC16};
use crate::structs::prog_structs_24::Struct103;
use crate::mem_funcs::mem_ops_1::get_fn_ptr_at_address;

pub fn get_gui_dc_1018_4db0(struct_param_1: &mut Struct199, in_win_handle: HWND16) -> HDC16 {
    struct_param_1.field_0x16 = in_win_handle;
    let dev_ctx_handle = GetDC16(in_win_handle);
    // (struct_param_1 + 0x14) = dev_ctx_handle;
    struct_param_1.field_0x14 = dev_ctx_handle;
    return dev_ctx_handle;
}

pub unsafe fn create_drawing_dc_1018_4e04(
    ctx: &mut AppContext,
    param_1: &mut  Struct115,
    param_2: u16,
    param_3: u16,
    param_4: u16,
) {
    let pp_var1: fn();
    let mut dc_handle: HDC16 = 0;
    let mut hdc_ptr: u16;
    // let local_bx_8: &mut  Struct115;
    let mut u_var2: u16;
    let mut u_var3: u32;
    let mut local_10: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut func_ptr: u32;
    let mut inner_struct: Struct103 = Struct103::new();
    let mut offset: u16;

  // u_var2 = (param_1  >> 0x10);
  //   local_bx_8 = param_1;
    pp_var1 = (param_1 + 0x14);
    (**pp_var1)();
    u_var3 = process_struct_1008_4772(&mut param_1.field_0xa);
    pass1_1008_43cc(&mut param_1.field_0xa);
  // dc_handle = CreateDC16(u_var3, (u_var3  >> 0x10), 0x0, 0x42340000);
    dc_handle = CreateDC16(0x42340000, 0x0, u_var3, u_var3);
    param_1.dc_handle = dc_handle;
    hdc_ptr = param_1.dc_handle;
    inner_struct = param_1.field_0xa.clone();
    func_ptr = (param_1.field_0xa.field_0x8);
    let func = get_fn_ptr_at_address(func_ptr);
    func(
        inner_struct,
        hdc_ptr,
        u_var2,
        param_1,
        param_2,
    );
    param_1.field_0x1a = hdc_ptr;
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

pub unsafe fn get_dc_1020_921c(ctx: &mut AppContext, param_1: &mut Struct622) {
    let var1: HDC16;
    let mut var3: u16;
    let var4: &mut  Struct622;
    let var5: &mut  Struct622;
    let var2: &mut Struct2551;
    let mut var6: u32;
    let mut var7: u16;
    let mut var8: u16;

  // u_var1 = (in_stack_0000ffe2  >> 0x10);
  // local_es_4 = (param_1  >> 0x10);
    var4 = param_1;
    param_1.field_0x0 = ctx.s_1_1050_389a;
    param_1.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    param_1.field_0x0 = (ctx.ctx.s_18_2_1050_3aa5 + 3);
    param_1.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    param_1.window_handle_0x4 = param_2;
    param_1.field_0x0 = ctx.s_0_020_1050_3ab0;
    param_1.field_0x2 = ctx.PTR_LOOP_1050_1008;
    param_1.u16_0x6 = 0;
    param_1.field_0xa = 0;
    param_1.field_0xc = 0;
    param_1.field_0xe = 0;
    param_1.field_0x10 = 0;
    param_1.i16_0x12 = 0;
    param_1.field_0x0 = 0x96c8;
    param_1.field_0x2 = 0x1020;
    var1 = GetDC16(param_1.window_handle_0x4.field_0x0);
    param_1.field_0xa = var1;
    var2 = process_struct_1010_20ba(&mut ctx.g_struct_1050_0ed0, CONCAT22(var3, 0x48));
  // u_var1 = (ppVar2  >> 0x10);
    param_1.field_0xc = (var2 + 10);
    param_1.field_0xe = (var2 + 0xc);
    return;
}

pub unsafe fn get_dc_1020_717e(ctx: &mut AppContext, param_1: &mut  u16, param_2: u32) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let pu_var3: &mut  u32;
    let pu_var4: &mut  u16;
    let pu_var5: &mut  u32;

    let mut i32_var6: i32;
    let mut u_var7: u16;
    let unaff_ss: u8;
    let pp_var8: &mut  Struct2551;
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
  // u_var12 = (param_1  >> 0x10);
    get_sys_metrics_1020_7c1a(i_var11, u_var12, param_2, (param_2 >> 0x10));
    (i_var11 + 0x14) = 0;
    (i_var11 + 0x18) = param_2;
    (i_var11 + 0x1c) = 0;
    (i_var11 + 0x20) = 0;

    *param_1 = 0x754c;

    (i_var11 + 2) = 0x1020;
    pp_var8 =
        process_struct_1010_20ba(&mut ctx.g_struct_1050_0ed0, CONCAT22(in_stack_0000ffdc, 4));
  // u_var7 = (pp_var8  >> 0x10);
    (i_var11 + 0x1c) = pp_var8;
    (i_var11 + 0x1e) = u_var7;
    pp_var1 = ((i_var11 + 0x1c) + 4);
    (**pp_var1)(0x10, (i_var11 + 0x1c), u_var7, 0, i_var11, u_var12);
    local_4 = GetDC16((i_var11 + 4));
    u_var2 = (i_var11 + 0x1c);
    (u_var2 + 0x178) = local_4;
    u_var2 = (i_var11 + 0x1c);
  // u_var7 = (u_var2  >> 0x10);
    i32_var6 = u_var2;
    pu_var5 = (i32_var6 + 0x24);
    u_var9 = SUB41(pu_var5, 0);
    u_var10 = (pu_var5 >> 8);
    pp_var1 = (*pu_var5 + 0x14);
    (**pp_var1)(0x38, u_var9, (i32_var6 + 0x26));
    pp_var8 = process_struct_1010_20ba(
        &mut ctx.g_struct_1050_0ed0,
        CONCAT13(u_var10, CONCAT12(u_var9, 0x29)),
    );
    u_var2 = (pp_var8 + 0xe);
    pu_var3 = u_var2;
    pass1_1008_4d84((pu_var5 & 0xffff | ctx.dx_reg << 0x10), pu_var3);
    pu_var4 = &local_4;
    palette::realize_palette_1008_4e08(pu_var3, u_var2, pu_var4, ctx.stack_seg_reg);
    (i_var11 + 0x20) = pu_var4;
}
