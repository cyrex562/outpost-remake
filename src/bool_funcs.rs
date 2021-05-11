use crate::defines::AppContext;
use crate::app_context::AppContext;

pub fn check_flag_1000_1ab0(ctx: &mut AppContext, param_1: u16) -> u16 {
    let mut u_var1: u16;
    let mut u_var2: u16;

    if ctx.ax_reg == 0x2000 {
        return 0x2000;
    }
    if in_ax < 0xfff0 {
        if ctx.ax_reg < 0x1001 {
            return 0x1000;
        }
        u_var1 = 0x2000;
        if ctx.ax_reg < (ctx.s_571_bmp_1050_1fff[2..]) {
            while {
                u_var2 = u_var1;
                u_var1 = u_var2 >> 1;
                ctx.ax_reg <= u_var1
            } {}
            return u_var2 & 0xfffe;
        }
        while (u_var1 = u_var1 * 2, u_var1 != 0) {
            if in_ax <= u_var1 {
                return (u_var1 + 0x10 & !(u_var1 < 0xfff0)) - 0x10;
            }
        }
    }
    return 0xfff0;
}

pub fn check_and_clear_global_1000_1f68(ctx: &mut AppContext) {
    ctx.PTR_LOOP_1050_5f26 = ctx.PTR_LOOP_1050_5f26 + -1;
    if (ctx.PTR_LOOP_1050_5f26 < 0) {
        ctx.PTR_LOOP_1050_5f26 = 0x0;
    }
    return;
}

// WARNING: Removing unreachable block (ram,0x10002018)

pub fn check_and_set_global_1000_1fea(ctx: &mut AppContext) -> u16 {
    let pu_var1: *mut u8;
    let mut b_var2: bool;

    pu_var1 = ctx.PTR_LOOP_1050_5f22 + 1;
    b_var2 = ctx.PTR_LOOP_1050_5f22 == 0x0;
    ctx.PTR_LOOP_1050_5f22 = pu_var1;
    if ((b_var2) && ((ctx.g_alloc_addr_1050_5f20 | ctx.g_astruct_94_1050_5f1e) != 0)) {
        ctx.PTR_LOOP_1050_5f22 = &ctx.dos_alloc_addr_1050_0002;
    }
    return 1;
}
