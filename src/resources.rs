use crate::defines::{U32Ptr, Struct_1010_4b3e, Struct18};
use crate::fn_ptr::fn_ptr_1000::fn_ptr_1000_17ce;
use crate::global::AppContext;
use crate::pass::pass_1010::pass1_1010_1d80;
use crate::util::read_string_from_addr;
use crate::win_struct::{HANDLE16, HGLOBAL16, HMODULE16, HRSRC16};
use crate::winapi::{FindResource16, FreeResource16, GlobalUnlock16, LoadResource16, WIN16_LockResource16};

pub fn find_n_load_rsrc_1010_4e9e(ctx: &mut AppContext, param_1: u32, handle_2: HGLOBAL16) {
    let b_var1: bool;
    let rsrc_handle: HRSRC16;
    let i_var2: i16;
    let u_var3: u16;
    let handle_1: HGLOBAL16;

    // u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    if (i_var2 + 0x20) != 0x0 {
        handle_1 = handle_2;
        if (i_var2 + 0x2a) != 0x0 {
            handle_1 = ctx.s_tile2_bmp_1050_1538 as HGLOBAL16;
            b_var1 = GlobalUnlock16(handle_2);
            if b_var1 == false {
                handle_1 = ctx.s_tile2_bmp_1050_1538;
                FreeResource16(ctx.s_tile2_bmp_1050_1538 as HANDLE16);
            }
        }
        rsrc_handle = FindResource16(
            handle_1,
            &read_string_from_addr(ctx.PTR_LOOP_1050_000a),
            None
        );
        handle_1 = LoadResource16(ctx.s_tile2_bmp_1050_1538 as HMODULE16, rsrc_handle);
        (i_var2 + 0x2a) = handle_1;
        if handle_1 != 0x0 {
            WIN16_LockResource16(ctx.s_tile2_bmp_1050_1538 as HGLOBAL16);
            return;
        }
    }
    return;
}


pub fn free_rsrc_1010_4b3e(
    ctx: &mut AppContext,
    param_1: &mut Struct18,
    handle_2: HGLOBAL16,
    unaff_ss: u16
) {
    let pi_var1: U32Ptr;
    let pu_var2: u32;
    let u_var3: u16;
    let ppc_var4: u32;
    let pu_var5: u32;
    let u_var6: u32;
    let unlocked: bool;
    let struct_2: &mut Struct18;
    let i_var9: i16;
    let u_var10: u16;
    let u_var11: u16;
    let handle_1: HGLOBAL16;
    let i_stack4: i16;

    // u_var10 = (param_1 >> 0x10);
    struct_2 = param_1;
    param_1.field_0x0 = (ctx.s_SCForceMorale__s_for_colony__08l_1050_5024 + 0x6);
    struct_2.field_0x2 = 0x1010;
    handle_1 = handle_2;
    if struct_2.field_0x2a != 0x0 {
        handle_1 = ctx.s_tile2_bmp_1050_1538 as HGLOBAL16;
        unlocked = GlobalUnlock16(handle_2);
        if unlocked == false {
            handle_1 = ctx.s_tile2_bmp_1050_1538 as HGLOBAL16;
            FreeResource16(ctx.s_tile2_bmp_1050_1538 as HGLOBAL16);
        }
    }
    (struct_2.field_0x2a) = 0x0;
    if *(struct_2.field_0x12) != 0x0 {
        i_stack4 = 0x0;
        loop {
            pu_var5 = (struct_2.field_0x12);
            pi_var1 = (pu_var5 + 0x8);
            if *pi_var1 == i_stack4 || *pi_var1 < i_stack4 { break; }
            u_var11 = (*pu_var5);
            i_var9 = *pu_var5;
            pu_var2 = (i_var9 + i_stack4 * 0x4);
            u_var3 = (i_var9 + i_stack4 * 0x4 + 0x2);
            if (u_var3 | pu_var2) != 0x0 {
                ppc_var4 = *pu_var2;
                (**ppc_var4)(handle_1, pu_var2, u_var3, 0x1);
            }
            i_stack4 += 0x1;
        }
    }
    u_var6 = (struct_2.field_0x12);
    fn_ptr_1000_17ce(ctx, (u_var6 + 0x4), 0x1000);
    fn_ptr_1000_17ce(ctx, struct_2.field_0x12, 0x1000);
    pu_var2 = struct_2.field_0x16;
    u_var3 = struct_2.field_0x18;
    if (u_var3 | pu_var2) != 0x0 {
        ppc_var4 = *pu_var2;
        (**ppc_var4)(0x1000, pu_var2, u_var3, 0x1);
    }
    fn_ptr_1000_17ce(ctx, (struct_2.field_0x1a), 0x1000);
    pass1_1010_1d80(param_1, unaff_SS);
    return;
}
