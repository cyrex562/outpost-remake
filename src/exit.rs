use crate::winapi::{FatalAppExit16, FatalExit};

pub fn exit_1000_25cc(param_1: i16, param_2: u16, param_3: u16) -> U32Ptr {
    let pi_var1: U32Ptr;
    let mut pcVar2: String;
    let mut str: String;
    let pi_var3: U32Ptr;
    let pi_var4: U32Ptr;
    let mut pc_var5: String;
    let i_var6: i16;
    let i_var7: i16;

    i_var7 = 0x2;
    i_var6 = 0x2;
    pass1_1000_25a8(param_2, param_3);
    pass1_1000_2913(i_var6, param_2, param_3);
    str = poss_str_op_1000_28dc(i_var7);
    if (str != 0x0) {
        i_var6 = 0x9;
        if (*str == 'M') {
            i_var6 = 0xf;
        }
        str = str + i_var6;
        i_var6 = 0x22;
        pc_var5 = str;
        loop {
            if (i_var6 == 0x0) {
                break;
            }
            i_var6 += -0x1;
            pcVar2 = pc_var5;
            pc_var5 = pc_var5 + 0x1;
            if *pcVar2 == '\r' {
                break;
            }
        }
        pc_var5[-0x1] = '\0';
    }
    FatalAppExit16(param_3, str);
    FatalExit();
    pi_var4 = &ctx.PTR_LOOP_1050_63fe;
    loop {
        piVar1 = pi_var4;
        pi_var4 = pi_var4 + 0x1;
        i_var6 = *piVar1;
        pi_var3 = pi_var4;
        if ((i_var6 == param_1) || (pi_var3 = (i_var6 + 0x1), pi_var3 == 0x0)) {
            return pi_var3;
        }
        i_var6 = -0x1;
        loop {
            if (i_var6 == 0x0) {
                break;
            }
            i_var6 += -0x1;
            piVar1 = pi_var4;
            pi_var4 = (pi_var4 + 0x1);
            if *pivar1 == '\0' {
                break;
            }
        }
    }
}

// WARNING (jumptable): Unable to track spacebase fully for stack
// WARNING: Unable to track spacebase fully for stack
// WARNING: Variable defined which should be unmapped: param_2
// WARNING: Variable defined which should be unmapped: param_1

pub unsafe fn exit_1000_25f2(
    param_1: u16,
    param_2: u16,
    param_3: i16,
    param_4: i16,
    param_5: u16,
    param_6: u16,
    param_7: u16,
) -> U32Ptr {
    i16 * *ppiVar1;
    let piVar2: U32Ptr;
    let mut pcVar3: String;
    let puVar4: U32Ptr;
    let piVar5: U32Ptr;
    let uVar6: u16;
    let mut str: String;
    let iVar7: i16;
    let piVar8: U32Ptr;
    let mut pcVar9: String;

    puVar4 = (param_4 + 0x1 & 0xfffe);
    if ((puVar4 < &param_3)
        && (
            piVar5 = -(puVar4 + -&param_3),
            ppiVar1 = &ctx.PTR_LOOP_1050_000a,
            *ppiVar1 < piVar5 || *ppiVar1 == piVar5,
        ))
    {
        ppiVar1 = &ctx.PTR_LOOP_1050_000c;
        if (piVar5 <= *ppiVar1 && *ppiVar1 != piVar5) {
            &ctx.PTR_LOOP_1050_000c = piVar5;
        }
        piVar5[-0x1] = param_2;
        piVar5[-0x2] = param_1;
        return piVar5;
    }
    uVar6 = pass1_1000_29dc(param_7);
    if (0x5fce != -0x1) {
        // WARNING: Could not recover jumptable at 0x10002622. Too many
        // branches
        // WARNING: Treating indirect jump as call
        piVar5 = (*0x5fce)();
        return piVar5;
    }
    pass1_1000_25a8(param_5, param_6);
    pass1_1000_2913(0x0, param_5, param_6);
    str = poss_str_op_1000_28dc(0x0);
    if (str != 0x0) {
        iVar7 = 0x9;
        if (*str == 'M') {
            iVar7 = 0xf;
        }
        str = str + iVar7;
        iVar7 = 0x22;
        pcVar9 = str;
        loop {
            if (iVar7 == 0x0) {
                break;
            }
            iVar7 += -0x1;
            pcVar3 = pcVar9;
            pcVar9 = pcVar9 + 0x1;
            if *pcVar3 == '\r' {
                break;
            }
        }
        pcVar9[-0x1] = '\0';
    }
    FatalAppExit16(param_6, str);
    FatalExit();
    piVar5 = &ctx.PTR_LOOP_1050_63fe;
    loop {
        piVar2 = piVar5;
        piVar5 = piVar5 + 0x1;
        iVar7 = *piVar2;
        piVar8 = piVar5;
        if ((iVar7 == param_3) || (piVar8 = (iVar7 + 0x1), piVar8 == 0x0)) {
            return piVar8;
        }
        iVar7 = -0x1;
        loop {
            if (iVar7 == 0x0) {
                break;
            }
            iVar7 += -0x1;
            piVar2 = piVar5;
            piVar5 = (piVar5 + 0x1);
            if *piVar2 == '\0' {
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
