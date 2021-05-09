use crate::{
    defines::AppContext,
    mem_funcs::alloc_mem_1000_167a,
    pass_funcs::{pass1_fn_1000_25a8, pass1_fn_1000_2913},
    string_funcs::process_string_1000_28dc,
    sys_funcs::{FatalAppExit16, FatalExit},
};

pub unsafe fn exit_1000_25cc(ctx: &mut AppContext) -> *mut i32 {
    let piVar1: *mut i32;
    let pc_var2: *mut libc::c_char;
    let i_var3: i32;
    let piVar4: *mut i32;
    let pi_var5: *mut i32;
    let pcVar6: *mut libc::c_char;
    let mut in_stack_00000008: i32;
    let pcVar7: *mut libc::c_char;

    pcVar7 = &ctx.dos_alloc_addr_1050_0002;
    pass1_fn_1000_25a8();
    pass1_fn_1000_2913();
    pcVar7 = process_string_1000_28dc(pcVar7);
    if (pcVar7 != 0x0) {
        i_var3 = 9;
        let val = *pcVar7;
        if (val == 'M') {
            i_var3 = 0xf;
        }
        pcVar7 = pcVar7 + i_var3;
        i_var3 = 0x22;
        pcVar6 = pcVar7;
        while {
            if i_var3 == 0 {
                break;
            }
            i_var3 = i_var3 + -1;
            pc_var2 = pcVar6;
            pcVar6 = pcVar6 + 1;
            let val = *pc_var2;
            val != '\r'
        } {}
        pcVar6[-1] = '\0';
    }
    // FatalAppExit16(CONCAT22(0x1050, pcVar7), 0);
    FatalAppExit16(pcVar7, 0);
    FatalExit(0xff);
    pi_var5 = (ctx.s___NMSG___1050_63f6 + 8);
    loop {
        piVar1 = pi_var5;
        pi_var5 = pi_var5 + 1;
        let val = *piVar1;
        i_var3 = val;
        piVar4 = pi_var5;
        if ((i_var3 == in_stack_00000008) || (piVar4 = (i_var3 + 1), piVar4 == 0x0)) {
            return piVar4;
        }
        i_var3 = -1;
        while {
            if (i_var3 == 0) {
                break;
            }
            i_var3 = i_var3 + -1;
            piVar1 = pi_var5;
            pi_var5 = (pi_var5 + 1);
            let val = *piVar1;
            val != '\0'
        } {}
    }
}

pub unsafe fn exit_1000_2950(ctx: &mut AppContext, param_1: i32) -> *mut i32 {
    let piVar1: *mut i32;
    let pc_var2: *mut libc::c_char;
    let pu_var3: *mut u8;
    let pcVar4: *mut libc::c_char;
    let mut i_var5: i32;
    let pi_var6: *mut i32;

    let pi_var7: *mut i32;
    let in_cx: u16;
    let mut in_dx: u16;
    let pcVar8: *mut libc::c_char;

    pu_var3 = ctx.PTR_LOOP_1050_6066;
    ctx.PTR_LOOP_1050_6066 = &ctx.PTR_LOOP_1050_1000;
    pi_var7 = alloc_mem_1000_167a(in_ax, in_dx);
    ctx.PTR_LOOP_1050_6066 = pu_var3;
    if ((in_dx | pi_var7) != 0) {
        return pi_var7;
    }
    pass1_fn_1000_25a8(ctx);
    pass1_fn_1000_2913(ctx);
    pcVar4 = process_string_1000_28dc(in_cx);
    if (pcVar4 != 0x0) {
        i_var5 = 9;
        let val = *pcVar4;
        if (val == 'M') {
            i_var5 = 0xf;
        }
        pcVar4 = pcVar4 + i_var5;
        i_var5 = 0x22;
        pcVar8 = pcVar4;
        while {
            if (i_var5 == 0) {
                break;
            }

            i_var5 = i_var5 + -1;
            pc_var2 = pcVar8;
            pcVar8 = pcVar8 + 1;
            let val = *pc_var2;
            val != '\r'
        } {}
        pcVar8[-1] = '\0';
    }
    // FatalAppExit16(CONCAT22(0x1050, pcVar4), 0);
    FatalAppExit16(pcVar4, 0);
    FatalExit(0xff);
    pi_var7 = ctx.s___NMSG___1050_63f6 + 8;
    loop {
        piVar1 = pi_var7;
        pi_var7 = pi_var7 + 1;
        let val = *piVar1;
        i_var5 = val;
        pi_var6 = pi_var7;
        if ((i_var5 == param_1) || (pi_var6 = (i_var5 + 1), pi_var6 == 0x0)) {
            return pi_var6;
        }
        i_var5 = -1;
        while {
            if i_var5 == 0 {
                break;
            }
            i_var5 = i_var5 + -1;
            piVar1 = pi_var7;
            pi_var7 = (pi_var7 + 1);
            let val = *piVar1;
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
    FatalAppExit16(ctx.s_ABNORMAL_PROGRAM_TERMINATION_1050_6544, 0);
    return;
}
