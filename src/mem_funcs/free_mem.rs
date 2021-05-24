use crate::app_context::AppContext;
use crate::err_ops::invoke_error_handler_1000_1e61;
use crate::mem_funcs;
use crate::mem_funcs::mem_ops_1::StructuredData;
use crate::mem_funcs::mem_size;
use crate::pass::pass_funcs::pass1_fn_1000_20a2;
use crate::struct_ops::struct_ops_1;
use crate::structs::my_structs::Pass110309e9cInput;
use crate::structs::prog_structs_20::Struct87;
use crate::structs::prog_structs_25::Struct86;
use crate::structs::prog_structs_26::{Struct98, Struct99};
use crate::structs::prog_structs_31::Struct100;
use crate::structs::prog_structs_8::Struct88;
use crate::util::{CONCAT12, CONCAT13, CONCAT22, ZEXT24};

// u32  free_mem_1000_0016(param_1: u32)
pub unsafe fn free_mem_1000_0016(ctx: &mut AppContext, param_1: &mut StructuredData) -> u32 {
    let mut u_var1: u32;
    // param_1 + 0x14
    let var_2 = param_1::get_u16(0x14);
    if var_2 != -0x4153 {
        invoke_error_handler_1000_1e61(ctx, ctx.code_seg_reg, 10, 0);
        return 0xffffffff;
    }
    u_var1 = free_mem_1000_0052(&ctx.g_alloc_addr_1050_1050);
    return u_var1;
}


// u32  free_mem_1000_0052()
pub fn free_mem_1000_0052(param_1: &mut  u32) -> u32 {
    // pu_var1: &mut  u32;
    let mut pu_var1: u32;
    // let mut u_var2: i32;
    let mut u_var2: u32;
    // let mut i_var3: i32;
    let mut i_var3: i32;
    // let mut u_var4: u32;
    let mut u_var4: u32;
    // let b_var5: bool;
    let mut b_var5: bool;
    // local_BX__1: &mut  Struct86;
    let mut local_BX__1: Struct86;
    // let mut local_16: u16;
    let mut local_16: u16;
    // let mut local_14: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let local_a: &mut  Struct87;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var2 = local_BX__1.field_0x1e;
    i_var3 = local_BX__1.field_0x20;
    local_8 = 0;
    while {
        local_a = (&local_BX__1.field_0x0 + local_8 * 2);
        if ((local_a != 0x0) && (local_8 != 3)) {
            local_e = 0;
            while {
                local_c = local_a.field_0x4;
                u_var4 = local_a.field_0x8;
                if ((u_var4 + 10) == 0) {
                    b_var5 = free_mem_1000_0510();
                    if (b_var5 == 0) {}
                    // goto LAB_1000_00f9;
                    if (local_c == local_a) {
                        local_c = 0;
                    }
                } else {
                    if (local_e == 0) {
                        local_e = local_a;
                    }
                }
                local_a = local_c;
                local_c != local_e
            } {}
        }
        local_8 = local_8 + 1;
        local_8 < 5
    } {}
    if (local_BX__1.field_0x32 != 0x0) {
        (*local_BX__1.field_0x32)();
    }
    // LAB_1000_00f9:
    pu_var1 = &local_BX__1.field_0x1e;
    return CONCAT22(
        (i_var3 - local_BX__1.field_0x20) - (u_var2 < *pu_var1),
        u_var2 - *pu_var1,
    );
}

pub fn free_mem_1000_0510() {
    let pu_var1: &mut  u32;
    let pi_var2: &mut  i32;
    let paVar3: &mut  Struct88;
    let mut u_var4: i32;
    let mut in_ax: i32;
    let mut u_var5: i32;
    let mut trunc_alloc_size: u16;
    let mut u_var6: i32;
    Struct88 * *in_bx;
    let mut bVar7: bool;
    let mut u_var8: u32;
    // ppu_var9: &mut  Vec<u8>;
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
    let temp_5f5b47e88b: &mut  Struct88;

    temp_5f5b47e88b = *in_bx;
    paVar3 = in_bx[1];
    u_var8 = mem_size::get_mem_sz_1000_1532(&ctx.g_alloc_addr_1050_1050);
  // trunc_alloc_size = (u_var8  >> 0x10);
    u_var5 = u_var8;
    ppu_var9 = &ctx.PTR_LOOP_1050_0000;
    if (in_ax != 0) {
        u_var4 = temp_5f5b47e88b.field_0x1e;
        u_var6 = (temp_5f5b47e88b.field_0x20 - trunc_alloc_size) - (u_var4 < u_var5);
        pu_var1 = &temp_5f5b47e88b.field_0x24;
        let pu_var1_val = unsafe { *pu_var1 };
        bVar7 = u_var6 < unsafe { *pu_var1 };
        if ((bVar7 || u_var6 == pu_var1_val)
            && (bVar7 || (u_var4 - u_var5 < temp_5f5b47e88b.field_0x22)))
        {
            bVar7 = false;
            // goto LAB_1000_0595;
        }
    }
    ppu_var9 = 0x1050057f;
    struct_ops_1::struct_func_1000_0368();
    pu_var1 = (s_version__d__d_1050_0012 + 0xc);
    unsafe {
        u_var4 = *pu_var1;
        *pu_var1 = *pu_var1 - u_var5;
        pi_var2 = s_New_failed_in_Op__Op_1050_0020;
        *pi_var2 = (*pi_var2 - trunc_alloc_size) - (u_var4 < u_var5);
    }
    bVar7 = true;
    // LAB_1000_0595:
    if (bVar7) {
        in_bx[6] = 0x0;
        free_mem_1000_13ce(ppu_var9 & 0xffff0000 | ZEXT24(in_bx));
        return 1;
    }
    return 0;
}

pub unsafe fn free_mem_1000_093a(ctx: &mut AppContext, param_1: &mut Pass110309e9cInput) -> bool {
    let var1: i32;

    if &PTR_LOOP_1050_000c != -0x352f {
        invoke_error_handler_1000_1e61(ctx, ctx.code_seg_reg, 0xe, 0);
        return false;
    }
    // let param_1_val = *param_1;
    // param_1_val = ctx.PTR_LOOP_1050_000e;
    *param_1 =  ctx.PTR_LOOP_1050_000e;
    if *param_1 == 0 {
        ctx.PTR_DAT_0005_0000_1050_0004 = 1;
    }
    ctx.PTR_LOOP_1050_000e = param_1.field_0x0.clone();
    var1 = ctx.PTR_LOOP_1050_000a;
    var1 = var1  - 1;
    if var1 == 0 {
        free_mem_1000_0510();
    }
    return true;
}

pub fn free_mem_1000_13ce(alloc_addr: u32) -> bool {
    let mut out_handle: u16;
    let mut alloc_handle: u32;
    let mut local_4: u16;

    alloc_handle = GlobalHandle16(alloc_addr);
    if (alloc_handle != 0) {
        out_handle = GlobalFree16(alloc_handle);
        return (out_handle == 0);
    }
    return 0;
}

pub fn free_mem_1000_15ce(param_1: &mut  u32, param_2: u16) {
    let pu_var1: &mut  u32;
    let mut u_var2: i32;
    let mut u_var3: i32;
    let mut local_8: u16;
    let mut local_4: u16;

    u_var3 = param_2 | param_1;
    while (u_var3 != 0) {
        unsafe { pu_var1 = *param_1 };
        u_var2 = param_1[1];
        GlobalDOSFree16(param_2);
        u_var3 = u_var2 | pu_var1;
        param_2 = u_var2;
        param_1 = pu_var1;
    }
    return;
}

pub unsafe fn free_mem_1000_1b68(param_1: &mut  Struct98, param_2: u16) -> bool {
    let b_var1: bool;
    let mut unaff_cs: u16;

    if (param_1.field_0x14 != -0x4153) {
        invoke_error_handler_1000_1e61(unaff_cs, 10, 0);
        return 0;
    }
    b_var1 = free_mem_1000_1b9a(0, CONCAT22(param_2, param_1));
    return b_var1;
}

pub unsafe fn free_mem_1000_1b9a(param_1: u16, param_1_00: &mut  Struct99) -> bool {
    let mut u_var1: i32;
    let mut u_var2: i32;
    let mut u_var3: u32;
    let mut u_var4: i32;
    let mut u_var5: i32;
    let local_bx_5: &mut  Struct99;
    let local_SI_34: &mut  Struct100;
    let mut u_var6: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

  // u_var6 = (param_1_00  >> 0x10);
    local_bx_5 = param_1_00;
    local_bx_5.field_0x14 = 0;
    local_4 = 0;
    while {
        local_SI_34 = (local_4 * 2);
        if (local_SI_34 != 0x0) {
            while {
                u_var3 = &local_SI_34.field_0x8;
                (u_var3 + 0xc) = 0;
                free_mem_1000_13ce(CONCAT22(local_SI_34.field_0xa, local_SI_34.field_0x8));
                local_SI_34 = local_SI_34.field_0x4;
                (local_4 * 2) != local_SI_34
            } {}
        }
        local_4 = local_4 + 1;
        local_4 < 5
    } {}
    u_var4 = local_bx_5.field_0x10;
    u_var5 = local_bx_5.field_0x12;
    while (true) {
        _local_8 = CONCAT22(u_var5, u_var4);
        if ((u_var5 | u_var4) == 0) {
            break;
        }
        u_var1 = *_local_8;
        u_var2 = (u_var4 + 2);
        free_mem_1000_13ce(CONCAT22(u_var5, u_var4));
        u_var4 = u_var1;
        u_var5 = u_var2;
    }
    pass1_fn_1000_20a2(param_1_00);
    free_mem_1000_13ce(param_1_00);
    return 1;
}

pub unsafe fn free_mem_1008_ad0c(param_1: u16, param_2: u8) {
    let in_stack_00000007: u8;
    let mut in_stack_00000008: u8;
    param_1 = CONCAT13(in_stack_00000007, CONCAT12(param_2, param_1));
    *param_1 = ctx.s_1_1050_389a;
    (param_1 + 2) = ctx.PTR_LOOP_1050_1008;
    if ((in_stack_00000008 & 1) != 0) {
        free_mem_1000_093a(param_1);
    }
    return _param_1 & 0xffff0000 | CONCAT12(param_2, param_1) & 0xffff;
}
