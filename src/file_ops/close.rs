use crate::app_context::AppContext;
use crate::err_ops::{error_check_1000_0dc6, error_check_1000_17ce};
use crate::file_ops::{misc, read};
use crate::mem_funcs::get_fn_ptr_at_address;
use crate::pass::pass14_funcs::{pass1_1008_6f7a, pass1_1008_7056};
use crate::prog_structs::prog_structs_25::Struct9;
use crate::prog_structs::prog_structs_28::FileObject;
use crate::prog_structs::prog_structs_2::Struct7;
use crate::typedefs::HFILE16;
use crate::winapi::_lclose16;

pub unsafe fn close_file_1008_496c(ctx: &mut AppContext, struct_param_2: &mut Struct7) {
    let pu_var1: u32;
    let mut u_var2: u32;
    let local_bx_5: Struct7;
    let mut local_res6: u16;
    let temp_86216c208fd: u32;
    let mut func_ptr: u32;
    let mut temp_5f096a4ace: u32;

    param_2.u16_field_0 = ctx.PTR_LOOP_1050_4c4c;
    struct_param_2.u16_fld_1 = ctx.PTR_LOOP_1050_1008;
    pu_var1 = struct_param_2.func_ptr_2;
    u_var2 = struct_param_2.u32_field_3;
    if (u_var2 | pu_var1) != 0 {
        let fn_to_call = get_fn_ptr_at_address(struct_param_2.func_ptr_2);
        fn_to_call();
    }
    error_check_1000_17ce(ctx, struct_param_2);
    if &struct_param_2.pv_buffer_0x1a != 0 {
        temp_5f096a4ace = struct_param_2.pv_buffer_0x1a;
        error_check_1000_0dc6(ctx, temp_5f096a4ace);
    }
    if (struct_param_2.hfile_field_5 != 0xffff) {
        _lclose16(struct_param_2.hfile_field_5);
    }
    struct_param_2._type.u16_field_0 = ctx.s_1_1050_389a.clone();
    struct_param_2.u16_fld_1 = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub unsafe fn close_file_1008_4c26(struct_param_1: &mut Struct7, param_2: u8) -> &mut Struct7 {
    close_file_1008_496c(ctx, struct_param_1);
    if (param_2 & 1) != 0 {
        error_check_1000_17ce(ctx, struct_param_1);
    }
    return struct_param_1;
}

pub unsafe fn close_file_1008_6dd0(param_1: *mut Struct9) {
    let local_bx_4: *mut Struct9;
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    if (local_bx_4.file != 0xffff) {
        _lclose16(local_bx_4.file);
        local_bx_4.file = 0xffff;
    }
    error_check_1000_17ce(ctx,param_1);
    return;
}

pub unsafe fn close_file_1008_6e78(ctx: &mut AppContext, struct_param_1: &mut FileObject) -> bool {
    let b_var1: bool;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut u_var4: u16;
    // let mut unaff_ss: u16;
    let mut local_4: [u8; 2];

    ctx.g_u16_1050_0310 = 0;
    b_var1 = read::read_from_file_1008_7146(ctx, struct_param_1);
    if b_var1 != 0 {
        // u_var4 = (struct_param_1 >> 0x10);
        // u_var3 = struct_param_1;
        // CONCAT22(unaff_ss, local_4)
        // (u_var3 + 4)
        // set_array_val_1008_72a8(local_4, struct_param_1 );
        // CONCAT22(unaff_ss, local_4)
        i_var2 = pass1_1008_7056(struct_param_1, &mut local_4);
        if (i_var2 != 0)
            && (
                i_var2 = pass1_1008_6f7a(struct_param_1, &mut local_4),
                i_var2 != 0,
            )
        {
            b_var1 = close_file_1008_726c(ctx, struct_param_1);
            if (b_var1 == 0) {
                return 0;
            }
            return 1;
        }
        _lclose16((u_var3 + 4));
    }
    return 0;
}

pub fn close_file_1008_726c(ctx: &mut AppContext, param_1: &mut FileObject) -> bool {
    let mut file_handle: HFILE16;
    if param_1.file != 0xffff {
        file_handle = _lclose16(param_1.file);
        if file_handle == 0xffff {
            ctx.g_u16_1050_0310 = 0x6d1;
            return false;
        }
        param_1.file = 0xffff;
        ctx.g_u16_1050_0310 = 0;
    }
    return true;
}
