use crate::{
    defines::AppContext,
    mem_funcs::alloc_mem_1000_167a,
    pass_funcs::{pass1_fn_1000_25a8, pass1_fn_1000_2913},
    string_funcs::process_string_1000_28dc,
};
use crate::app_context::AppContext;
use crate::winapi_funcs::{FatalAppExit16, FatalExit};

pub unsafe fn exit_1000_25cc(ctx: &mut AppContext) -> *mut i32 {
    let mut p_var1: u32 = 0;
    let mut pc_var2: String;
    let mut u32_var3: u32 = 0;
    let mut p_var4: u32 = 0;
    let mut p_var5: u32 = 0;
    let mut string_6: String;
    // let mut i_var8: u32 = 0;
    let mut string_7: String;

    string_7 = String::from_utf8(ctx.dos_alloc_addr_1050_0002.clone()).unwrap();
    pass1_fn_1000_25a8(ctx);
    pass1_fn_1000_2913(ctx, 0);
    string_7 = process_string_1000_28dc(ctx, &string_7);
    if string_7.len() != 0x0 {
        u32_var3 = 9;
        let val = string_7[0];
        if val == 'M' {
            u32_var3 = 0xf;
        }
        string_7 = string_7[u32_var3..];
        u32_var3 = 0x22;
        string_6 = string_7;
        while {
            if u32_var3 == 0 {
                break;
            }
            u32_var3 = u32_var3 - 1;
            pc_var2 = string_6;
            string_6 = string_6[1..].clone();
            let val = *pc_var2;
            val != '\r'
        } {}
        string_6[-1] = '\0';
    }
    // FatalAppExit16(CONCAT22(0x1050, pcVar7), 0);
    // TODO
    // FatalAppExit16(string_7, 0);
    FatalExit(0xff);
    // TODO
    // pi_var5 = (ctx.s___NMSG___1050_63f6 + 8);
    loop {
        p_var1 = p_var5;
        p_var5 = p_var5 + 1;
        let val = *p_var1;
        u32_var3 = val;
        p_var4 = p_var5;
        // if (u32_var3 == i_var8) || (p_var4 = (u32_var3 + 1)) == 0x0 {
        //     return p_var4;
        // }
        u32_var3 = -1;
        while {
            if u32_var3 == 0 {
                break;
            }
            u32_var3 = u32_var3 + -1;
            p_var1 = p_var5;
            p_var5 = (p_var5 + 1);
            let val = *p_var1;
            val != '\0'
        } {}
    }
}

pub unsafe fn exit_1000_2950(ctx: &mut AppContext, param_1: u32) -> Vec<u8> {
    let mut p_var1: Vec<u8>;
    let string_2: String;
    let mut i_var5: u32;
    let mut pi_var6: Vec<u8>;
    let mut string_8: String;

    let pu_var3 = ctx.PTR_LOOP_1050_6066;
    ctx.PTR_LOOP_1050_6066 = &ctx.PTR_LOOP_1050_1000;
    let mut p_var7 = alloc_mem_1000_167a(ctx.ax_reg, in_dx);
    ctx.PTR_LOOP_1050_6066 = pu_var3;
    if (ctx.dx_reg | p_var7) != 0 {
        return p_var7.clone();
    }
    pass1_fn_1000_25a8(ctx);
    pass1_fn_1000_2913(ctx, 0);
    let mut string_4 = process_string_1000_28dc(ctx, ctx.cx_reg);
    if string_4.len() != 0x0 {
        i_var5 = 9;
        let val = string_4[0];
        if val == 'M' {
            i_var5 = 0xf;
        }
        string_4 = string_4[i_var5..];
        i_var5 = 0x22;
        string_8 = string_4.clone();
        while {
            if i_var5 == 0 {
                break;
            }

            i_var5 = i_var5 - 1;
            string_2 = string_8.clone();
            string_8 = string_8[1..].clone();
            let val = string_2[0];
            val != '\r'
        } {}
        string_8[-1] = '\0';
    }
    // FatalAppExit16(CONCAT22(0x1050, pcVar4), 0);
    FatalAppExit16(0, &string_4);
    FatalExit(0xff);
    // pi_var7 = ctx.s___NMSG___1050_63f6 + 8;
    loop {
        p_var1 = p_var7.clone();
        p_var7 = p_var7[1..].clone();
        let val = p_var1[0];
        i_var5 = val.into();
        pi_var6 = p_var7.clone();
        // if (i_var5 == param_1) || (pi_var6 = (i_var5 + 1), pi_var6.len() == 0x0) {
        //     return pi_var6;
        // }
        i_var5 = -1;
        while {
            if i_var5 == 0 {
                break;
            }
            i_var5 = i_var5 + -1;
            p_var1 = p_var7;
            p_var7 = p_var7[1..].clone();
            let val = *p_var1;
            val != '\0'
        } {}
    }
}

pub fn return0_1000_29ef() -> u16 {
    return 0;
}

pub fn return_1000_39e1() {
    return;
}

// WARNING: Removing unreachable block (ram,0x10003afe)
// WARNING: Removing unreachable block (ram,0x10003a40)
// WARNING: Removing unreachable block (ram,0x10003b7e)
pub fn fatal_app_exit_1000_3e9e(ctx: &mut AppContext) {
    FatalAppExit16(0,ctx.s_ABNORMAL_PROGRAM_TERMINATION_1050_6544);
    return;
}
