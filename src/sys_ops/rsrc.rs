use crate::app_context::AppContext;
use crate::err_ops::error_check_1000_17ce;
use crate::mem_funcs::get_fn_ptr_at_address;
use crate::pass::pass8_funcs::pass1_1010_1d80;
use crate::prog_structs::prog_structs_2::{Struct131, Struct7};
use crate::prog_structs::prog_structs_8::Struct60;
use crate::typedefs::{HGLOBAL16, SEGPTR};

pub unsafe fn free_rsrc_1010_4b3e(ctx: &mut AppContext, param_1: &mut Struct7) {
    let pu_var1: u16;
    let pu_var2: u32;
    let fn_ptr_var4: fn();
    let pu_var5: &Struct131;
    let b_var7: bool;
    let mut local_4: u16;

    param_1.u16_fld_0 = (ctx.s_SCForceMorale__s_for_colony__08l_1050_5024 + 6);
    param_1.u16_fld_1 = 0x1010; // (i_var8 + 2)
    if param_1.u16_field_0x2a != 0 { // i_var8 + 0x2a
        // unaff_cs = offset;
        b_var7 = GlobalUnlock16(param_1.u16_field_0x2a); // (i_var8 + 0x2a)
        if b_var7 == false {
            // unaff_cs = SUB42(offset, 0);
            FreeResource16(param_1.u16_field_0x2a); // (i_var8 + 0x2a)
        }
    }
    param_1.u16_field_0x2a = 0; //(i_var8 + 0x2a) = 0;
    if param_1.struct_field_0x12 != 0 { // (i_var8 + 0x12)
        local_4 = 0;
        loop {
            pu_var5 = &param_1.struct_field_0x12;
            pu_var1 = pu_var5.field_0x8;  //(pu_var5 + 8);
            unsafe {
                if pu_var1 == local_4 || pu_var1 < local_4 {
                    break;
                }
                // u_var11 = (*pu_var5 >> 0x10);
                // i_var9 = *pu_var5;

                pu_var2 = (pu_var5.field_0x0 + local_4 * 4) as u32;
                u_var3 = (pu_var5.field_0x0 + local_4 * 4 + 2);
                if (u_var3 | pu_var2) != 0 {
                    fn_ptr_var4 = get_fn_ptr_at_address(pu_var2);
                    (fn_ptr_var4)(ctx.code_seg_reg, pu_var2, u_var3, 1);
                }
            }
            local_4 = local_4 + 1;
        }
    }
    // u_var6 = (i_var8 + 0x12);
    let mut u_var6 = &mut param_1.struct_field_0x12;
    error_check_1000_17ce(ctx, &mut u_var6.field_0x4);
    error_check_1000_17ce(ctx, &mut param_1.struct_field_0x12);
    pu_var2 = param_1.u32_field_0x16;
    // u_var3 = (i_var8 + 0x18);
    if (pu_var2) != 0 {
        fn_ptr_var4 = get_fn_ptr_at_address(pu_var2);
        fn_ptr_var4(0x1000, pu_var2, 1);
    }
    error_check_1000_17ce(ctx, &mut param_1.pv_buffer_0x1a); // (i_var8 + 0x1a)
    pass1_1010_1d80(param_1);
    return;
}

pub fn load_rsrc_1010_4e9e(in_struct_1: &mut Struct60) -> SEGPTR {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let mut unlock_result: u16;
    let mut h_resource: u16;
    let mut handle: HGLOBAL16;
    let mut u_var3: u16;
    let mut SVar4: SEGPTR;

    u_var3 = (in_struct_1 >> 0x10);
    local_bx_5 = in_struct_1;
    if local_bx_5.field_0x20 != 0 {
        if local_bx_5.resource_handle != 0 {
            unlock_result = GlobalUnlock16(local_bx_5.resource_handle);
            if unlock_result == 0 {
                FreeResource16(local_bx_5.resource_handle);
            }
        }
        u_var1 = local_bx_5.field_0x12;
        u_var2 = (u_var1 + 4);
        h_resource = FindResource16(
            0xa,
            *((u_var2 + local_bx_5.field_0x20 * 2) * 2 + 0x1384),
            ctx.g_h_instance_1050_038c,
        );
        handle = LoadResource16(h_resource, ctx.g_h_instance_1050_038c);
        local_bx_5.resource_handle = handle;
        if (handle != 0) {
            SVar4 = LockResource16(handle);
            return SVar4;
        }
    }
    return 0;
}
