use crate::defines::U32Ptr;
use crate::{FatalAppExit16, FatalExit};
use crate::global::AppContext;
use crate::pass::pass_1000::{pass1_1000_25a8, pass1_1000_2913, pass1_1000_29dc};
use crate::string::string_1000::str_op_1000_28dc;
use crate::winapi::{FatalAppExit16, FatalExit};

pub fn exit_1000_25cc(ctx: &mut AppContext, param_1: i16, param_2: u16, action: u16) -> U32Ptr {
    let pi_var1: U32Ptr;
    let mut string_1: String;
    let mut reason: String;
    let pi_var3: U32Ptr;
    let pi_var4: U32Ptr;
    let mut string_2: String;

    let mut i_var7 = 0x2;
    let mut i_var6 = 0x2;
    pass1_1000_25a8(ctx, param_2, action);
    pass1_1000_2913(ctx, i_var6, param_2, action);
    reason = str_op_1000_28dc(ctx, i_var7);
    if reason != 0x0 {
        i_var6 = 0x9;
        if reason[0] == 'M' {
            i_var6 = 0xf;
        }
        reason = reason[i_var6..];
        i_var6 = 0x22;
        string_2 = reason;
        loop {
            if i_var6 == 0x0 {
                break;
            }
            i_var6 += -0x1;
            string_1 = string_2;
            string_2 = string_2[0x1..].to_string();
            if *string_1 == '\r' {
                break;
            }
        }
        // string_2[-0x1] = '\0';
    }
    FatalAppExit16(action, &reason);
    FatalExit();
    pi_var4 = ctx.PTR_LOOP_1050_63fe;
    loop {
        pi_var1 = pi_var4;
        pi_var4 = pi_var4 + 0x1;
        i_var6 = *pi_var1;
        pi_var3 = pi_var4;
        if (i_var6 == param_1) || (pi_var3 = (i_var6 + 0x1), pi_var3 == 0x0) {
            return pi_var3;
        }
        i_var6 = -0x1;
        loop {
            if i_var6 == 0x0 {
                break;
            }
            i_var6 += -0x1;
            pi_var1 = pi_var4;
            pi_var4 = (pi_var4 + 0x1);
            if *pi_var1 == '\0' {
                break;
            }
        }
    }
}

// WARNING (jumptable): Unable to track spacebase fully for stack
// WARNING: Unable to track spacebase fully for stack
// WARNING: Variable defined which should be unmapped: param_2
// WARNING: Variable defined which should be unmapped: param_1

pub fn exit_1000_25f2(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: u16,
    param_3: i16,
    param_4: i16,
    param_5: u16,
    param_6: u16,
    param_7: &String,
) -> U32Ptr {
    let ppi_var: u32;
    let string_4: String;
    let mut string_2: String;
    let pu_var4: U32Ptr;
    let pi_var5: U32Ptr;
    let string_5: &String;
    let mut string_1: String;
    let i_var7: i16;
    let pi_var8: U32Ptr;
    let mut string_3: String;

    pu_var4 = (param_4 + 0x1 & 0xfffe);
    pi_var5 = -(pu_var4 + -&param_3);
    ppi_var = ctx.PTR_LOOP_1050_000a;
    if (pu_var4 < param_3) && (*ppi_var < pi_var5 || *ppi_var == pi_var5) {
        ppi_var = ctx.PTR_LOOP_1050_000c;
        if pi_var5 <= *ppi_var && *ppi_var != pi_var5 {
            ctx.PTR_LOOP_1050_000c = pi_var5;
        }
        pi_var5[-0x1] = param_2;
        pi_var5[-0x2] = param_1;
        return pi_var5;
    }
    string_5 = pass1_1000_29dc(param_7);
    if 0x5fce != -0x1 {
        pi_var5 = (*0x5fce)();
        return pi_var5;
    }
    pass1_1000_25a8(ctx, param_5, param_6);
    pass1_1000_2913(ctx, 0x0, param_5, param_6);
    string_1 = str_op_1000_28dc(ctx, 0x0);
    if string_1 != 0x0 {
        i_var7 = 0x9;
        if *string_1 == 'M' {
            i_var7 = 0xf;
        }
        string_1 = string_1[i_var7..].to_string();
        i_var7 = 0x22;
        string_3 = string_1;
        loop {
            if i_var7 == 0x0 {
                break;
            }
            i_var7 += -0x1;
            string_2 = string_3;
            string_3 = string_3[0x1..].to_string();
            if *string_2 == '\r' {
                break;
            }
        }
        string_3[-0x1] = '\0';
    }
    FatalAppExit16(param_6, &string_1);
    FatalExit();
    pi_var5 = ctx.PTR_LOOP_1050_63fe;
    loop {
        string_4 = pi_var5;
        pi_var5 = pi_var5 + 0x1;
        i_var7 = *string_4;
        pi_var8 = pi_var5;
        if (i_var7 == param_3) || (pi_var8 = (i_var7 + 0x1), pi_var8 == 0x0) {
            return pi_var8;
        }
        i_var7 = -0x1;
        loop {
            if i_var7 == 0x0 {
                break;
            }
            i_var7 += -0x1;
            string_4 = pi_var5;
            pi_var5 = (pi_var5 + 0x1);
            if *string_4 == '\0' {
                break;
            }
        }
    }
}

pub fn fatal_app_exit_1000_3e9e(ctx: &mut AppContext, app_exit_action: u16) {
    FatalAppExit16(
        app_exit_action,
        ctx.s_ABNORMAL_PROGRAM_TERMINATION_1050_6544,
    );
    return;
}
