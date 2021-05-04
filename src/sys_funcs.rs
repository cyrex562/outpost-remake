use crate::defines::{ATOM, HWND16, LOGPALETTE, LPARAM, LRESULT, PAINTSTRUCT16, WPARAM16};

pub unsafe fn get_module_file_name_1000_262c(ctx: &mut AppContext, param_1: *mut u8) {
    let pc_var1: *mut libc::c_char;
    let mut c_var2: u8;
    let pu_var3: *mut u8;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut u_var6: i32;
    let mut u_var7: i32;
    let mut ppc_var8: *mut *mut libc::c_char;
    let pc_var9: *mut libc::c_char;
    let pc_var10: *mut libc::c_char;
    let pc_var11: *mut libc::c_char;
    let mut u_var12: u16;
    let unaff_ss: *mut libc::c_char;
    let mut u_var13: u32;
    let mut u_var14: u32;
    let in_stack_00000000: *mut u8;
    let mut u_stack12: u16;
    let mut i_stack8: i32;
    let pc_stack6: *mut libc::c_char;
    let apu_stack4: [u8; 2];

    ctx.PTR_LOOP_1050_5fd4 = param_1;
    param_1 = (s_fem15_wav_1050_263a + 3);
    ctx.PTR_LOOP_1050_5fd2 = in_stack_00000000;
    u_var13 = exit_1000_2950();
    apu_stack4[1] = g_h_instance;
    i_stack8 = 0x104;
    u_stack12 = SUB42(s_fem17_wav_1050_264e + 7, 0);
    ctx.PTR_LOOP_1050_5fc4 = (u_var13 >> 0x10);
    u_var14 = u_var13;
    i_var4 = GetModuleFileName16();
    u_var14._2_2_ = (u_var14 >> 0x10);
    ctx.PTR_LOOP_1050_5fc2 = u_var14;
    param_1 = (u_var13 >> 0x10);
    pc_stack6 = u_var13;
    *(i_stack8 + i_var4) = 0;
    i_var4 = 1;
    ctx.PTR_LOOP_1050_5fb8 = (&ctx.PTR_LOOP_1050_0000 + 1);
    pc_var9 = (ctx.s_New_failed_in_Op__Op__DialogHand_1050_0073 + 0xe);
    // LAB_1000_266c:
    while {
        while {
            pc_var1 = pc_var9;
            pc_var9 = pc_var9 + 1;
            c_var2 = *pc_var1;

            Var2 == ' '
        } {}
        c_var2 == '\t'
    } {}
    if ((c_var2 != '\r') && (c_var2 != '\0')) {
        PTR_LOOP_1050_5fb8 = PTR_LOOP_1050_5fb8 + 1;
        loop {
            pc_var9 = pc_var9 + -1;
            // LAB_1000_267f:
            pc_var1 = pc_var9;
            pc_var9 = pc_var9 + 1;
            c_var2 = *pc_var1;
            if ((c_var2 == ' ') || (c_var2 == '\t')) {}
            // goto LAB_1000_266c;
            if ((c_var2 == '\r') || (c_var2 == '\0')) {
                break;
            }
            if (c_var2 == '\"') {
                // LAB_1000_26b8:
                while {
                    loop {
                        loop {
                            pc_var1 = pc_var9;
                            pc_var9 = pc_var9 + 1;
                            c_var2 = *pc_var1;
                            if ((c_var2 == '\r') || (c_var2 == '\0')) {}
                            // goto LAB_1000_26e8;
                            if (c_var2 == '\"') {}
                            // goto LAB_1000_267f;
                            if (c_var2 == '\\') {
                                break;
                            }
                            i_var4 = i_var4 + 1;
                        }
                        u_var6 = 0;
                        while {
                            pc_var11 = pc_var9;
                            u_var6 = u_var6 + 1;
                            pc_var9 = pc_var11 + 1;
                            c_var2 = *pc_var11;
                            c_var2 == '\\'
                        } {}
                        if (c_var2 == '\"') {
                            break;
                        }
                        i_var4 = i_var4 + u_var6;
                        pc_var9 = pc_var11;
                    }
                    i_var4 = i_var4 + (u_var6 >> 1) + ((u_var6 & 1) != 0);
                    (u_var6 & 1) != 0
                } {}
                // goto LAB_1000_267f;
            }
            if (c_var2 != '\\') {
                i_var4 = i_var4 + 1;
                // goto LAB_1000_267f;
            }
            u_var6 = 0;
            while {
                u_var6 = u_var6 + 1;
                pc_var1 = pc_var9;
                pc_var9 = pc_var9 + 1;
                c_var2 = *pc_var1;
                c_var2 == '\\'
            } {}
            if (c_var2 == '\"') {
                i_var4 = i_var4 + (u_var6 >> 1) + ((u_var6 & 1) != 0);
                if ((u_var6 & 1) == 0) {}
                // goto LAB_1000_26b8;
                // goto LAB_1000_267f;
            }
            i_var4 = i_var4 + u_var6;
        }
    }
    // LAB_1000_26e8:
    pc_stack6 = &ctx.g_alloc_addr_1050_1050;
    i_var4 = -((ctx.PTR_LOOP_1050_5fb8 + (ctx.PTR_LOOP_1050_5fb8 + 1) * 4 + i_var4 + 1) & 0xfffe);
    ctx.PTR_LOOP_1050_5fba = (&pc_stack6 + i_var4 + 2);
    pc_var11 = (&pc_stack6 + (ctx.PTR_LOOP_1050_5fb8 + 1) * 4 + i_var4 + 2);
    ctx.PTR_LOOP_1050_5fbc = unaff_ss;
    *(&pc_stack6 + i_var4) = unaff_ss;
    pu_var3 = ctx.PTR_LOOP_1050_5fc4;
    u_var12 = *(&pc_stack6 + i_var4);
    (&pc_stack6 + i_var4 + 2) = ctx.PTR_LOOP_1050_5fc2;
    (&pc_stack6 + i_var4 + 4) = pu_var3;
    ppc_var8 = (&stack0x0000 + i_var4);
    *(&pc_stack6 + i_var4) = (&pc_stack6 + i_var4 + 2);
    (&i_stack8 + i_var4) = offset;
    (&stack0xfff6 + i_var4) = (ctx.s_fem37_wav_1050_2716 + 9);
    u_var5 = exported_stub_1000_29dc();
    u_var5 = &ctx.PTR_LOOP_1050_5f7e;
    pc_var9 = (ctx.s_New_failed_in_Op__Op__DialogHand_1050_0073 + 0xe);
    // LAB_1000_272e:
    while {
        while {
            pc_var1 = pc_var9;
            pc_var9 = pc_var9 + 1;
            c_var2 = *pc_var1;
            c_var2 == ' '
        } {}
        c_var2 == '\t'
    } {}
    if ((c_var2 == '\r') || (c_var2 == '\0')) {
        // LAB_1000_27c1:
        *(&pc_stack6 + i_var4) = offset;
        (&i_stack8 + i_var4) = (s_fem54_wav_1050_27c0 + 5);
        u_var5 = exported_stub_1000_29dc();
        *ppc_var8 = 0x0;
        ppc_var8[1] = 0x0;
        // WARNING: Could not recover jumptable at 0x100027d2. Too many branches
        // WARNING: Treating indirect jump as call
        (**&PTR_LOOP_1050_5fd2)();
        _PTR_LOOP_1050_5fc2 = CONCAT22(PTR_LOOP_1050_5fc4, PTR_LOOP_1050_5fc2);
        return;
    }
    *ppc_var8 = pc_var11;
    ppc_var8[1] = unaff_ss;
    ppc_var8 = ppc_var8 + 2;
    loop {
        pc_var9 = pc_var9 + -1;
        // LAB_1000_274f:
        pc_var1 = pc_var9;
        pc_var9 = pc_var9 + 1;
        c_var2 = *pc_var1;
        if ((c_var2 == ' ') || (c_var2 == '\t')) {
            pc_var1 = pc_var11;
            pc_var11 = pc_var11 + 1;
            *pc_var1 = '\0';
            // goto LAB_1000_272e;
        }
        if ((c_var2 == '\r') || (c_var2 == '\0')) {
            // LAB_1000_27be:
            *pc_var11 = '\0';
            // goto LAB_1000_27c1;
        }
        pc_var10 = pc_var9;
        if (c_var2 == '\"') {
            // LAB_1000_278b:
            loop {
                pc_var9 = pc_var10 + 1;
                c_var2 = *pc_var10;
                if ((c_var2 == '\r') || (c_var2 == '\0')) {}
                // goto LAB_1000_27be;
                if (c_var2 == '\"') {
                    break;
                }
                if (c_var2 == '\\') {
                    u_var6 = 0;
                    while {
                        pc_var10 = pc_var9;
                        u_var6 = u_var6 + 1;
                        pc_var9 = pc_var10 + 1;
                        c_var2 = *pc_var10;
                        c_var2 == '\\'
                    } {}
                    if (c_var2 == '\"') {
                        u_var7 = u_var6 >> 1;
                        while (u_var7 != 0) {
                            u_var7 = u_var7 - 1;
                            pc_var1 = pc_var11;
                            pc_var11 = pc_var11 + 1;
                            *pc_var1 = '\\';
                        }
                        {}
                        if ((u_var6 & 1) == 0) {
                            break;
                        }
                        pc_var1 = pc_var11;
                        pc_var11 = pc_var11 + 1;
                        *pc_var1 = '\"';
                        pc_var10 = pc_var9;
                    } else {
                        while (u_var6 != 0) {
                            u_var6 = u_var6 - 1;
                            pc_var1 = pc_var11;
                            pc_var11 = pc_var11 + 1;
                            *pc_var1 = '\\';
                        }
                        {}
                    }
                } else {
                    pc_var1 = pc_var11;
                    pc_var11 = pc_var11 + 1;
                    *pc_var1 = c_var2;
                    pc_var10 = pc_var9;
                }
            }
            // goto LAB_1000_274f;
        }
        if (c_var2 != '\\') {
            pc_var1 = pc_var11;
            pc_var11 = pc_var11 + 1;
            *pc_var1 = c_var2;
            // goto LAB_1000_274f;
        }
        u_var6 = 0;
        while {
            u_var6 = u_var6 + 1;
            pc_var1 = pc_var9;
            pc_var9 = pc_var9 + 1;
            c_var2 = *pc_var1;
            c_var2 == '\\'
        } {}
        if (c_var2 == '\"') {
            u_var7 = u_var6 >> 1;
            while (u_var7 != 0) {
                u_var7 = u_var7 - 1;
                pc_var1 = pc_var11;
                pc_var11 = pc_var11 + 1;
                *pc_var1 = '\\';
            }
            pc_var10 = pc_var9;
            if ((u_var6 & 1) == 0) {}
            // goto LAB_1000_278b;
            pc_var1 = pc_var11;
            pc_var11 = pc_var11 + 1;
            *pc_var1 = '\"';
            // goto LAB_1000_274f;
        }
        while (u_var6 != 0) {
            u_var6 = u_var6 - 1;
            pc_var1 = pc_var11;
            pc_var11 = pc_var11 + 1;
            *pc_var1 = '\\';
        }
    }
}

pub fn get_dos_env_1000_27d6() {
    let pi_var1: *mut i32;
    let pc_var2: *mut libc::c_char;
    let pi_var3: *mut i32;
    let mut cVar4: u8;
    char * *ppcVar5;
    let mut i_var6: i32;
    let mut i_var7: i32;
    let pi_var8: *mut i32;
    let pi_var9: *mut i32;
    let pc_var10: *mut libc::c_char;
    let pi_var11: *mut i32;
    let mut bVar12: bool;
    let mut dos_env: u32;
    let mut u_var13: u32;
    let mut u_var14: u32;
    let pc_var15: *mut libc::c_char;
    let mut uVar16: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;

    uVar16 = SUB42(&g_alloc_addr_1050_1050, 0);
    dos_env = GetDOSEnviornment16();
    dos_env._2_2_ = (dos_env >> 0x10);
    if (dos_env != 0) {
        dos_env._2_2_ = 0;
    }
    i_var7 = 0;
    pc_var10 = 0x0;
    i_var6 = -1;
    if (dos_env._2_2_ != 0) {
        cVar4 = *0x0;
        while (cVar4 != '\0') {
            while {
                if (i_var6 == 0) {
                    break;
                }
                i_var6 = i_var6 + -1;
                pc_var2 = pc_var10;
                pc_var10 = pc_var10 + 1;
                unsafe { *pc_var2 != '\0' }
            } {}
            i_var7 = i_var7 + 1;
            pc_var2 = pc_var10;
            pc_var10 = pc_var10 + 1;
            unsafe {
                cVar4 = *pc_var2;
            }
        }
    }
    u_var13 = exit_1000_2950(uVar16);
    u_var14 = exit_1000_2950();
    pc_var15 = (u_var13 >> 0x10);
    pc_var10 = u_var13;
    uVar16 = (u_var14 >> 0x10);
    PTR_LOOP_1050_5fbe = u_var14;
    pi_var8 = 0x0;
    loop {
        PTR_LOOP_1050_5fc0 = (u_var14 >> 0x10);
        ppcVar5 = u_var14;
        if (i_var7 == 0) {
            *ppcVar5 = 0x0;
            ppcVar5[1] = 0x0;
            return;
        }
        unsafe {
            bVar12 = *pi_var8 == s__C_FILE_INFO__1050_5f5c._0_2_;
        }
        if (bVar12) {
            pi_var11 = s__C_FILE_INFO__1050_5f5c;
            i_var6 = 6;
            pi_var9 = pi_var8;
            while {
                if (i_var6 == 0) {
                    break;
                }
                i_var6 = i_var6 + -1;
                pi_var3 = pi_var11;
                pi_var11 = pi_var11 + 1;
                pi_var1 = pi_var9;
                pi_var9 = pi_var9 + 1;
                unsafe {
                    bVar12 = *pi_var1 == *pi_var3;
                }
                bVar12
            } {}
            if (!bVar12) {}
            // goto LAB_1000_2867;
        } else {
            // LAB_1000_2867:
            *ppcVar5 = pc_var10;
            ppcVar5[1] = pc_var15;
            u_var14 = CONCAT22(PTR_LOOP_1050_5fc0, ppcVar5 + 2);
        }
        while {
            PTR_LOOP_1050_5fc0 = (u_var14 >> 0x10);
            pi_var1 = pi_var8;
            pi_var8 = (pi_var8 + 1);
            unsafe {
                cVar4 = *pi_var1;
            }
            pc_var2 = pc_var10;
            pc_var10 = pc_var10 + 1;
            unsafe {
                *pc_var2 = cVar4;
            }
            u_var14 = u_var14 & 0xffff | ZEXT24(PTR_LOOP_1050_5fc0) << 0x10;
            cVar4 != '\0'
        } {}
        i_var7 = i_var7 + -1;
    }
}

pub fn get_dos_env_1000_27da() {
    let pi_var1: *mut i32;
    let pc_var2: *mut libc::c_char;
    let pi_var3: *mut i32;
    let mut cVar4: u8;
    let mut in_AX: u16;
    let mut i_var5: i32;
    let mut i_var6: i32;
    let mut u_var7: u16;
    char * *ppc_var8;
    let mut i_var9: i32;
    let pi_var10: *mut i32;
    let pi_var11: *mut i32;
    let pc_var12: *mut libc::c_char;
    let pi_var13: *mut i32;
    let mut bVar14: bool;
    let SVar15: SEGPTR;
    let mut uVar16: u32;
    let pc_var17: *mut libc::c_char;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;

    SVar15 = GetDOSEnviornment16();
    i_var6 = (SVar15 >> 0x10);
    if (SVar15 != 0) {
        i_var6 = 0;
    }
    i_var9 = 0;
    pc_var12 = 0x0;
    i_var5 = -1;
    if (i_var6 != 0) {
        cVar4 = *0x0;
        while (cVar4 != '\0') {
            while {
                if (i_var5 == 0) {
                    break;
                }
                i_var5 = i_var5 + -1;
                pc_var2 = pc_var12;
                pc_var12 = pc_var12 + 1;
                unsafe { *pc_var2 != '\0' }
            } {}
            i_var9 = i_var9 + 1;
            pc_var2 = pc_var12;
            pc_var12 = pc_var12 + 1;
            unsafe {
                cVar4 = *pc_var2;
            }
        }
    }
    uVar16 = exit_1000_2950();
    pc_var17 = (uVar16 >> 0x10);
    pc_var12 = uVar16;
    uVar16 = exit_1000_2950();
    u_var7 = (uVar16 >> 0x10);
    ppc_var8 = uVar16;
    0x5fbe = ppc_var8;
    &PTR_LOOP_1050_5fc0 = u_var7;
    pi_var10 = 0x0;
    loop {
        if (i_var9 == 0) {
            *ppc_var8 = 0x0;
            ppc_var8[1] = 0x0;
            return;
        }
        unsafe {
            bVar14 = *pi_var10 == s__C_FILE_INFO__1050_5f5c;
        }
        if (bVar14) {
            pi_var13 = s__C_FILE_INFO__1050_5f5c;
            i_var5 = 6;
            pi_var11 = pi_var10;
            while {
                if (i_var5 == 0) {
                    break;
                }
                i_var5 = i_var5 + -1;
                pi_var3 = pi_var13;
                pi_var13 = pi_var13 + 1;
                pi_var1 = pi_var11;
                pi_var11 = pi_var11 + 1;
                unsafe {
                    bVar14 = *pi_var1 == *pi_var3;
                }
                bVar14
            } {}
            if (!bVar14) {}
            // goto LAB_1000_2867;
        } else {
            // LAB_1000_2867:
            *ppc_var8 = pc_var12;
            ppc_var8[1] = pc_var17;
            ppc_var8 = ppc_var8 + 2;
        }
        while {
            pi_var1 = pi_var10;
            pi_var10 = (pi_var10 + 1);
            unsafe {
                cVar4 = *pi_var1;
            }
            pc_var2 = pc_var12;
            pc_var12 = pc_var12 + 1;
            unsafe {
                *pc_var2 = cVar4;
            }
            cVar4 != '\0'
        } {}
        i_var9 = i_var9 + -1;
    }
}

pub fn dos3_call_1000_2bb6(uparam_1: i32, param_2: *mut Struct152) -> u32 {
    let pbVar1: *mut u8;
    let paVar2: *mut Struct152;
    let mut bVar3: u8;
    let mut i_var4: i32;
    let mut i_var5: i32;
    let mut in_dx: u16;
    let mut u_var6: i32;
    let mut unaff_BP: i32;
    let mut uStack4: u16;
    let mut iStack2: i32;

    paVar2 = param_2;
    iStack2 = unaff_BP + 1;
    uStack4 = SUB42(&g_alloc_addr_1050_1050, 0);
    bVar3 = param_2.field_0xa;
    if (((bVar3 & 0x82) != 0) && ((bVar3 & 0x40) == 0)) {
        param_2.field_0x4 = 0;
        if ((bVar3 & 1) != 0) {
            if ((bVar3 & 0x10) == 0) {}
            // goto LAB_1000_2c37;
            param_2.field_0x0 = param_2.field_0x6;
            bVar3 = bVar3 & 0xfe;
        }
        param_2.field_0xa = bVar3 & 0xef | 2;
        u_var6 = *&param_2.field_0xb;
        if (((bVar3 & 8) == 0)
            && ((bVar3 & 4) != 0
                || ((*&param_2.field_0xf0 & 1) == 0
                    && ((PTR_LOOP_1050_61ec != 0x0
                        && ((param_2 == 0x621c || (param_2 == 0x6228))
                            && ((*(u_var6 + 0x5f90) & 0x40) != 0)))
                        || (
                            process_struct_1000_2ce8(param_2, in_dx),
                            (paVar2.field_0xa & 8) == 0,
                        )))))
        {
            i_var4 = dos3_call_1000_39f2(u_var6, &param_1);
            i_var5 = 1;
        } else {
            i_var4 = paVar2.field_0x6;
            i_var5 = (paVar2).field_0x0 - i_var4;
            (paVar2).field_0x0 = i_var4 + 1;
            paVar2.field_0x4 = paVar2.field_0xf2 + -1;
            if (i_var5 == 0) {
                i_var4 = 0;
                if ((*(u_var6 + 0x5f90) & 0x20) != 0) {
                    dos3_call_1000_3636(u_var6, 0, 0, 2);
                    i_var4 = 0;
                    i_var5 = i_var4;
                }
            } else {
                i_var4 = dos3_call_1000_39f2(u_var6, paVar2.field_0x6, paVar2.field_0x8, i_var5);
            }
            **&paVar2.field_0x6 = param_1;
        }
        if (i_var4 == i_var5) {
            return param_1 & 0xff;
        }
    }
    // LAB_1000_2c37:
    pbVar1 = &paVar2.field_0xa;
    unsafe {
        *pbVar1 = *pbVar1 | 0x20;
    }
    return 0xffff;
}

// WARNING: Removing unreachable block (ram,0x10003622)

pub fn dos3_call_1000_35fe(uparam_1: i32) -> u16 {
    let pc_var1: *mut code;
    let mut u_var2: u16;
    let mut unaff_BP: i32;
    let mut bVar3: bool;

    if (param_1 < u16_1050_5f8a) {
        bVar3 = false;
        pc_var1 = swi(0x21);
        unsafe {
            u_var2 = (*pc_var1)(unaff_BP + 1);
        }
        if (!bVar3) {
            *(param_1 + 0x5f90) = 0;
        }
    } else {
        u_var2 = 0x900;
        bVar3 = true;
    }
    if (bVar3) {
        pass1_fn_1000_29b5(u_var2);
        return 0xffff;
    }
    return 0;
}

// WARNING: Variable defined which should be unmapped: local_8
// WARNING: Removing unreachable block (ram,0x100036b5)
// WARNING: Removing unreachable block (ram,0x10003681)
// WARNING: Removing unreachable block (ram,0x100036f7)
// WARNING: Removing unreachable block (ram,0x100036d8)

pub fn dos3_call_1000_3636(uparam_1: i32, uparam_2: i32, uparam_3: i32, uparam_4: i32) {
    let pbVar1: *mut u8;
    let pc_var2: *mut code;
    let mut u_var3: u16;
    let mut unaff_BP: i32;
    let mut bVar4: bool;
    let mut u_var5: u32;
    let mut local_8: u16;
    let mut local_6: u16;

    if (((param_1 < u16_1050_5f8a) || (PTR_LOOP_1050_61ec == 0x0)) || (2 < param_1)) {
        if ((PTR_LOOP_1050_6064 == 0x0) || ((param_3 & 0x8000) == 0)) {}
        // goto LAB_1000_36e3;
        if (param_4 == 0) {}
        // goto LAB_1000_369b;
        bVar4 = false;
        pc_var2 = swi(0x21);
        unsafe {
            u_var5 = (*pc_var2)();
        }
        u_var3 = u_var5;
        if (bVar4) {}
        // goto LAB_1000_299d;
        if ((param_4 & 2) == 0) {
            if (-1 < ((u_var5 >> 0x10) + param_3 + CARRY2(u_var3, param_2))) {
                // LAB_1000_36e3:
                bVar4 = false;
                pc_var2 = swi(0x21);
                unsafe {
                    u_var3 = (*pc_var2)();
                }
                if (!bVar4) {
                    pbVar1 = (param_1 + 0x5f90);
                    bVar4 = false;
                    unsafe {
                        *pbVar1 = *pbVar1 & 0xfd;
                    }
                }
                // goto LAB_1000_299d;
            }
        } else {
            pc_var2 = swi(0x21);
            unsafe {
                u_var5 = (*pc_var2)(unaff_BP + 1);
            }
            if (-1 < ((u_var5 >> 0x10) + param_3 + CARRY2(u_var5, param_2))) {}
            // goto LAB_1000_36e3;
            pc_var2 = swi(0x21);
            unsafe {
                (*pc_var2)();
            }
        }
        // LAB_1000_369b:
        u_var3 = s_471_bmp_1050_1600;
    } else {
        u_var3 = 0x900;
    }
    bVar4 = true;
    // LAB_1000_299d:
    if (bVar4) {
        pass1_fn_1000_29b5(u_var3);
    }
    return;
}

// WARNING: Variable defined which should be unmapped: local_8
// WARNING: Removing unreachable block (ram,0x10003989)
// WARNING: Removing unreachable block (ram,0x100038a1)
// WARNING: Removing unreachable block (ram,0x10003867)
// WARNING: Removing unreachable block (ram,0x10003967)
// WARNING: Removing unreachable block (ram,0x1000391a)
// WARNING: Removing unreachable block (ram,0x10003803)
// WARNING: Removing unreachable block (ram,0x100037b7)
// WARNING: Removing unreachable block (ram,0x10003799)
// WARNING: Removing unreachable block (ram,0x10003765)
// WARNING: Removing unreachable block (ram,0x100037ec)
// WARNING: Removing unreachable block (ram,0x100038d9)
// WARNING: Removing unreachable block (ram,0x100038f2)
// WARNING: Removing unreachable block (ram,0x1000393a)
// WARNING: Removing unreachable block (ram,0x1000384b)
// WARNING: Removing unreachable block (ram,0x1000388b)
// WARNING: Removing unreachable block (ram,0x100038ba)
// WARNING: Removing unreachable block (ram,0x100039b9)
// WARNING: Removing unreachable block (ram,0x1000381c)
// WARNING: Could not reconcile some variable overlaps

pub fn dos3_call_1000_370a(
    param_1: u16,
    param_2: u16,
    uparam_2: i32,
    param_3: u8,
    uparam_4: i32,
) -> u16 {
    let pc_var1: *mut code;
    let mut u_var2: i32;
    let mut u_var3: u16;
    let mut i_var4: i32;
    let mut b_var5: u8;
    let mut u_var6: i32;
    let mut extraout_dx: i32;
    let mut u_var7: u16;
    let mut unaff_BP: i32;
    let mut u_var8: i32;
    let mut bVar9: bool;
    let mut in_stack_0000fff2: i32;
    let local_8: u8;
    let local_7: u8;
    let local_6: u8;
    let mut local_5: u8;

    _param_3 = param_4;
    u_var2 = _local_6 & 0xff00;
    _local_6 = u_var2 | param_3;
    b_var5 = 0;
    if (((param_2 & 0x8000) == 0) && ((param_2 & 0x4000) != 0 || ((u8_1050_6061 & 0x80) == 0))) {
        b_var5 = 0x80;
    }
    bVar9 = false;
    pc_var1 = swi(0x21);
    u_var6 = param_2;
    unsafe {
        u_var3 = (*pc_var1)();
    }
    if (bVar9) {
        if ((u_var3 == 2) && ((u_var6 & 0x100) != 0)) {
            _local_8 = b_var5;
            _local_6 = (s_____s__lu_1050_38cd + 3);
            return_1000_39e1();
            u_var6 = 0;
            _param_3 = param_4;
            // LAB_1000_38e3:
            bVar9 = false;
            pc_var1 = swi(0x21);
            unsafe {
                u_var3 = (*pc_var1)();
            }
            if (bVar9) {}
            // goto LAB_1000_299d;
            if ((local_6 != '\0')
                || (
                    u_var7 = u_var3,
                    u_var8 = in_stack_0000fff2,
                    (param_2 & 2) == 0,
                ))
            {
                pc_var1 = swi(0x21);
                unsafe {
                    (*pc_var1)();
                }
                bVar9 = false;
                pc_var1 = swi(0x21);
                unsafe {
                    u_var3 = (*pc_var1)();
                }
                if (bVar9) {}
                // goto LAB_1000_299d;
                u_var7 = u_var3;
                u_var8 = _local_6;
                if (((_local_8 & 0x100) == 0) && ((_param_3 & 1) != 0)) {
                    u_var6 = (u_var6 | 1);
                    bVar9 = false;
                    pc_var1 = swi(0x21);
                    unsafe {
                        u_var3 = (*pc_var1)();
                    }
                    u_var8 = unaff_BP + 1;
                    if (bVar9) {}
                    // goto LAB_1000_299d;
                }
            }
            // LAB_1000_3973:
            if ((_local_8 & 0x40) == 0) {
                pc_var1 = swi(0x21);
                unsafe {
                    (*pc_var1)();
                }
                b_var5 = 0;
                if ((u_var6 & 1) != 0) {
                    b_var5 = 0x10;
                }
                if ((param_2 & 8) != 0) {
                    b_var5 = b_var5 | 0x20;
                }
            } else {
                b_var5 = 0;
            }
            if (u_var7 < &u16_1050_5f8a) {
                *(u_var7 + 0x5f90) = b_var5 | local_8 | 1;
                return u_var7;
            }
            pc_var1 = swi(0x21);
            unsafe {
                (*pc_var1)();
            }
            u_var3 = 0x1800;
        }
    } else {
        if ((u_var6 & 0x500) != 0x500) {
            _local_8 = CONCAT11(1, b_var5);
            pc_var1 = swi(0x21);
            unsafe {
                (*pc_var1)();
            }
            if ((extraout_dx & 0x80) != 0) {
                _local_8 = _local_8 | 0x40;
            }
            u_var7 = u_var3;
            u_var8 = _local_6;
            if ((_local_8 & 0x40) == 0) {
                if ((param_2 & 0x200) == 0) {
                    if (((_local_8 & 0x80) != 0) && ((param_2 & 2) != 0)) {
                        pc_var1 = swi(0x21);
                        unsafe {
                            (*pc_var1)();
                        }
                        pc_var1 = swi(0x21);
                        unsafe {
                            i_var4 = (*pc_var1)();
                        }
                        if ((i_var4 != 0) && (local_5 = (u_var2 >> 8), local_5 == '\x1a')) {
                            pc_var1 = swi(0x21);
                            unsafe {
                                (*pc_var1)(unaff_BP + 1);
                            }
                            pc_var1 = swi(0x21);
                            unsafe {
                                (*pc_var1)();
                            }
                        }
                        u_var6 = 0;
                        pc_var1 = swi(0x21);
                        unsafe {
                            (*pc_var1)();
                        }
                        u_var7 = u_var3;
                        u_var8 = in_stack_0000fff2;
                    }
                } else {
                    if ((param_2 & 3) == 0) {
                        unsafe {
                            pc_var1 = swi(0x21);
                            (*pc_var1)();
                            pc_var1 = swi(0x21);
                            (*pc_var1)();
                        }
                        // goto LAB_1000_38e3;
                    }
                    u_var6 = 0;
                    pc_var1 = swi(0x21);
                    unsafe {
                        (*pc_var1)();
                    }
                    u_var7 = u_var3;
                }
            }
            // goto LAB_1000_3973;
        }
        pc_var1 = swi(0x21);
        unsafe {
            (*pc_var1)();
        }
        u_var3 = 0x1100;
    }
    bVar9 = true;
    // LAB_1000_299d:
    if (bVar9) {
        pass1_fn_1000_29b5(u_var3);
        u_var3 = 0xffff;
    }
    return u_var3;
}

pub fn dos3_call_1000_39f2(param_1: *mut char, param_2: *mut char, param_3: u16) -> u16 {
    let pc_var1: *mut libc::c_char;
    let mut u_var2: i32;
    let pcVar3: *mut code;
    let mut u_var4: u16;
    let mut cVar5: u8;
    let mut u_var6: u16;
    let pcVar7: *mut libc::c_char;
    let mut u_var8: i32;
    let pc_var9: *mut libc::c_char;
    let mut i_var10: i32;
    let pu_var11: *mut u16;
    let mut unaff_BP: i32;
    let mut unaff_si: u16;
    let pc_var12: *mut libc::c_char;
    let pc_var13: *mut libc::c_char;
    let mut u_var14: u16;
    let mut unaff_cs: u16;
    let mut unaff_ss: u16;
    let uVar15: u8;
    let mut bVar16: u8;
    let mut in_af: u8;
    let mut bVar17: bool;
    let mut cVar18: u8;
    let mut cVar19: u8;
    let mut u_var20: u32;
    let mut uStack18: u16;
    let mut uStack16: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut uStack4: u16;
    let mut iStack2: i32;

    iStack2 = unaff_BP + 1;
    uStack4 = SUB42(&g_alloc_addr_1050_1050, 0);
    local_e = u16_1050_5f8a;
    pcVar7 = u16_1050_5f8a;
    if ((PTR_LOOP_1050_61ec != 0x0)
        && (
            pcVar7 = PTR_s_ed_in_Op_Op_1050_0028_1050_5f8e,
            param_1 < (&dos_alloc_addr_1050_0002 + 1),
        ))
    {
        param_1 = u16_1050_5f8a;
    }
    if (pcVar7 <= param_1) {
        uVar15 = true;
        u_var6 = 0x900;
        // goto LAB_1000_299d;
    }
    pcVar7 = param_1;
    if ((param_1[0x5f90] & 0x20) != 0) {
        uVar15 = false;
        pcVar3 = swi(0x21);
        unsafe {
            u_var6 = (*pcVar3)();
        }
        unaff_cs = 0x1000;
        if (uVar15) {}
        // goto LAB_1000_299d;
    }
    pc_var12 = param_2;
    if ((pcVar7[0x5f90] & 0x80) == 0) {
        // LAB_1000_3acf:
        uVar15 = false;
        u_var6 = param_3;
        if (param_3 != 0) {
            local_c = &g_alloc_addr_1050_1050;
            uVar15 = pcVar7 < local_e;
            if (uVar15) {
                uVar15 = 0;
                pcVar3 = swi(0x21);
                unsafe {
                    u_var20 = (*pcVar3)();
                }
            } else {
                local_e = s_2_3_1050_3b71;
                u_var20 = process_string_1000_55b1();
            }
            u_var6 = u_var20;
            local_a = param_2._2_2_;
            if (uVar15) {
                u_var6 = CONCAT11(9, u_var20);
            } else {
                uVar15 = false;
                if (u_var6 == 0) {
                    if (((pcVar7[0x5f90] & 0x40) == 0) || (*(u_var20 >> 0x10) != '\x1a')) {
                        uVar15 = true;
                        u_var6 = 0x1c00;
                    } else {
                        uVar15 = false;
                    }
                }
            }
        }
    } else {
        local_a = &g_alloc_addr_1050_1050;
        bVar17 = true;
        local_6 = 0;
        local_8 = 0;
        local_c = &local_e;
        u_var6 = param_3;
        pc_var13 = pc_var12;
        local_e = unaff_si;
        if (param_3 != 0) {
            while {
                if (u_var6 == 0) {
                    break;
                }
                u_var6 = u_var6 - 1;
                pc_var1 = pc_var13;
                pc_var13 = pc_var13 + 1;
                unsafe {
                    bVar17 = *pc_var1 == '\n';
                }
                !bVar17
            } {}
            if (!bVar17) {}
            // goto LAB_1000_3acf;
            pc_var9 = pc_var12;
            uStack18 = unaff_cs;
            uStack16 = param_2._2_2_;
            u_var8 = pass1_fn_1000_3bac();
            u_var4 = uStack16;
            if (u_var8 < 0xa9) {
                uStack18 = unaff_cs;
                uStack18 = bad_1000_25f2();
                if (pc_var13 == pc_var9) {
                    return local_e;
                }
                bVar16 = param_1 < local_e;
                pc_var12 = param_1 + -local_e;
                cVar19 = pc_var12 < 0;
                cVar18 = pc_var12 == 0x0;
                cVar5 = (POPCOUNT(pc_var12 & 0xff) & 1) == 0;
                if (bVar16) {
                    bVar16 = 0;
                    cVar19 = '\0';
                    cVar18 = 0x1;
                    cVar5 = 0x1;
                    pcVar3 = swi(0x21);
                    unsafe {
                        u_var8 = (*pcVar3)(&g_alloc_addr_1050_1050, u_var6, pcVar7);
                    }
                } else {
                    u_var8 = process_string_1000_55b1();
                }
                if (!bVar16) {
                    local_6 = local_6 + u_var8;
                    bVar16 = u_var6 < u_var8;
                    u_var2 = u_var6 - u_var8;
                    cVar19 = u_var2 < 0;
                    cVar18 = u_var2 == 0;
                    cVar5 = (POPCOUNT(u_var2 & 0xff) & 1) == 0;
                    if (bVar16 || cVar18) {
                        return local_e;
                    }
                }
                u_var2 = (cVar19 << 7 | cVar18 << 6 | in_af << 4 | cVar5 << 2 | 2 | bVar16) << 8;
                u_var6 = u_var8 & 0xff | u_var2;
                if (local_6 == 0) {
                    uVar15 = (u_var2 & 0x100) != 0;
                    if (uVar15) {
                        u_var6 = CONCAT11(9, (u_var8 & 0xff));
                    } else {
                        if (((param_1[0x5f90] & 0x40) == 0) || (unsafe { *param_2 != '\x1a' })) {
                            uVar15 = true;
                            u_var6 = 0x1c00;
                        } else {
                            uVar15 = false;
                        }
                    }
                    // goto LAB_1000_299d;
                }
            } else {
                pu_var11 = &uStack18 + 1;
                i_var10 = 0x200;
                if (u_var8 < 0x228) {
                    i_var10 = 0x80;
                }
                i_var10 = -i_var10;
                pcVar7 = (&uStack18 + i_var10 + 2);
                (&uStack18 + i_var10) = unaff_ss;
                u_var14 = (&uStack18 + i_var10);
                while {
                    pc_var1 = pc_var12;
                    pc_var12 = pc_var12 + 1;
                    unsafe {
                        cVar5 = *pc_var1;
                    }
                    if (cVar5 == '\n') {
                        cVar5 = '\r';
                        if (pcVar7 == pu_var11) {
                            (&uStack18 + i_var10) = (s_5_24_1050_3ab9 + 4);
                            cVar5 = dos3_call_1000_3ad9();
                        }
                        pc_var1 = pcVar7;
                        pcVar7 = pcVar7 + 1;
                        unsafe {
                            *pc_var1 = cVar5;
                        }
                        cVar5 = '\n';
                        local_8 = local_8 + 1;
                    }
                    if (pcVar7 == pu_var11) {
                        (&uStack18 + i_var10) = s_94_72_1050_3ac8;
                        cVar5 = dos3_call_1000_3ad9();
                    }
                    pc_var1 = pcVar7;
                    pcVar7 = pcVar7 + 1;
                    unsafe {
                        *pc_var1 = cVar5;
                    }
                    param_3 = param_3 - 1;
                    param_3 != 0
                } {}
                (&uStack18 + i_var10) = (s_0_020_1050_3ab0 + 1);
                dos3_call_1000_3ad9();
            }
        }
        uVar15 = local_6 < local_8;
        u_var6 = local_6 - local_8;
    }
    // LAB_1000_299d:
    if (uVar15) {
        local_c = s_fem102_wav_1050_29a2;
        pass1_fn_1000_29b5(u_var6);
        u_var6 = 0xffff;
    }
    return u_var6;
}

// WARNING: Unable to track spacebase fully for stack
// WARNING: Removing unreachable block (ram,0x10003afe)

pub fn dos3_call_1000_3ad9(param_1: u16) -> u16 {
    let pu_var1: *mut u32;
    let pi_var2: *mut i32;
    let mut i_var3: i32;
    let mut u_var4: i32;
    let pcVar5: *mut code;
    let mut u_var6: u16;
    let mut u_var7: i32;
    let mut in_CX: i32;
    let mut in_dx: i32;
    let mut u_var8: i32;
    let mut unaff_BP: i32;
    let mut unaff_DI: i32;
    let mut unaff_ss: u16;
    let mut bVar9: u8;
    let mut bVar10: bool;
    let mut cVar11: u8;
    let mut in_af: u8;
    let mut cVar12: u8;
    let mut cVar13: u8;

    if (unaff_DI == in_dx) {
        return param_1;
    }
    u_var8 = (unaff_BP + 6);
    pu_var1 = (unaff_BP + -0xc);
    unsafe {
        bVar9 = u_var8 < *pu_var1;
        u_var7 = u_var8 - *pu_var1;
    }
    cVar13 = u_var7 < 0;
    cVar12 = u_var7 == 0;
    cVar11 = (POPCOUNT(u_var7 & 0xff) & 1) == 0;
    if (bVar9) {
        bVar9 = 0;
        cVar13 = '\0';
        cVar12 = 0x1;
        cVar11 = 0x1;
        pcVar5 = swi(0x21);
        unsafe {
            u_var7 = (*pcVar5)(&g_alloc_addr_1050_1050);
        }
    } else {
        u_var7 = process_string_1000_55b1();
    }
    if (!bVar9) {
        pi_var2 = (unaff_BP + -4);
        unsafe {
            *pi_var2 = *pi_var2 + u_var7;
        }
        bVar9 = in_CX < u_var7;
        u_var4 = in_CX - u_var7;
        cVar13 = u_var4 < 0;
        cVar12 = u_var4 == 0;
        cVar11 = (POPCOUNT(u_var4 & 0xff) & 1) == 0;
        if (bVar9 || cVar12) {
            return param_1;
        }
    }
    u_var4 = (cVar13 << 7 | cVar12 << 6 | in_af << 4 | cVar11 << 2 | 2 | bVar9) << 8;
    u_var6 = u_var7 & 0xff | u_var4;
    if ((unaff_BP + -4) == 0) {
        bVar10 = (u_var4 & 0x100) != 0;
        if (bVar10) {
            u_var6 = CONCAT11(9, (u_var7 & 0xff));
        } else {
            if (((*(u_var8 + 0x5f90) & 0x40) == 0) || (**(unaff_BP + 8) != '\x1a')) {
                bVar10 = true;
                u_var6 = 0x1c00;
            } else {
                bVar10 = false;
            }
        }
    } else {
        u_var8 = (unaff_BP + -4);
        pu_var1 = (unaff_BP + -6);
        unsafe {
            bVar10 = u_var8 < *pu_var1;
            u_var6 = u_var8 - *pu_var1;
        }
    }
    i_var3 = (unaff_BP + -10);
    if (bVar10) {
        (i_var3 + 2) = s_fem102_wav_1050_29a2;
        pass1_fn_1000_29b5(u_var6, *(i_var3 + 2));
        u_var6 = 0xffff;
    }
    return u_var6;
}

pub fn dos3_call_1000_42de(param_1: u32, param_2: *mut u16, param_3: *mut u16) {
    let mut u_var1: u16;
    let mut u_var2: u16;
    let pcVar3: *mut code;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut unaff_BP: i32;
    let mut i_var6: i32;
    let mut unaff_si: u16;
    let mut i_var7: i32;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut bVar11: bool;
    let mut u_var12: u32;
    let mut u_var13: u16;

    i_var6 = unaff_BP + 1;
    u_var13 = SUB42(&g_alloc_addr_1050_1050, 0);
    u_var8 = (param_1 >> 0x10);
    i_var7 = param_1;
    u_var5 = (i_var7 + 2);
    u_var4 = (i_var7 + 4);
    u_var1 = (i_var7 + 8);
    u_var8 = (i_var7 + 10);
    u_var9 = (param_3 >> 0x10);
    unsafe {
        u_var10 = *param_3;
    }
    u_var2 = (param_3 + 6);
    bVar11 = false;
    pcVar3 = swi(0x21);
    unsafe {
        u_var12 = (*pcVar3)();
    }
    unsafe {
        *param_3 = u_var10;
    }
    (param_3 + 6) = u_var2;
    u_var10 = (param_2 >> 0x10);
    i_var7 = param_2;
    unsafe {
        *param_2 = u_var12;
    }
    (i_var7 + 2) = u_var5;
    (i_var7 + 4) = u_var4;
    (i_var7 + 6) = (u_var12 >> 0x10);
    (i_var7 + 8) = u_var1;
    (i_var7 + 10) = u_var8;
    if (bVar11) {
        pass1_fn_1000_29af(unaff_si, u_var13, i_var6);
    }
    (i_var7 + 0xc) = bVar11;
    return;
}

pub fn dos3_call_1000_435c(param_1: *mut u16) {
    let pc_var1: *mut code;
    let mut u_var2: u16;
    let mut in_CX: i32;
    let mut u_var3: i32;
    let mut extraout_dx: i32;
    let mut extraout_dx_00: i32;
    let mut u_var4: i32;
    let mut u_var5: i32;
    let mut cVar6: u8;

    pc_var1 = swi(0x21);
    unsafe {
        (*pc_var1)(&g_alloc_addr_1050_1050);
    }
    pc_var1 = swi(0x21);
    u_var3 = in_CX;
    u_var5 = extraout_dx;
    unsafe {
        (*pc_var1)();
    }
    cVar6 = u_var3;
    pc_var1 = swi(0x21);
    unsafe {
        (*pc_var1)(u_var3 >> 8);
    }
    u_var4 = extraout_dx_00;
    if ((u_var5 != extraout_dx_00) && (u_var4 = extraout_dx_00, cVar6 == '\x17')) {
        u_var3 = in_CX;
        u_var4 = u_var5;
    }
    u_var2 = pass1_fn_1000_462e(u_var3 - 0x7bc, u_var4 >> 8, u_var4 & 0xff);
    if (param_1._2_2_ != 0) {
        (param_1 + 2) = u_var4;
        unsafe {
            *param_1 = u_var2;
        }
    }
    return;
}

pub fn set_global_uint_1000_4d0c(uparam_1: i32) {
    UINT_1050_61e8 = param_1;
    PTR_LOOP_1050_61ea = 0;
    return;
}

pub fn dos3_call_1000_4f2e() -> u16 {
    let pc_var1: *mut code;
    let mut u_var2: u16;
    let mut unaff_BP: i32;
    let mut bVar3: bool;

    bVar3 = false;
    pc_var1 = swi(0x21);
    unsafe {
        u_var2 = (*pc_var1)(&g_alloc_addr_1050_1050, unaff_BP + 1);
    }
    if (bVar3) {
        pass1_fn_1000_29b5(u_var2);
        return 0xffff;
    }
    return 0;
}

pub fn dos3_call_1000_4f94() -> i32 {
    let pc_var1: *mut code;
    let mut b_var2: u8;
    let mut unaff_BP: i32;

    pc_var1 = swi(0x21);
    unsafe {
        b_var2 = (*pc_var1)(unaff_BP + 1);
    }
    return b_var2 + 1;
}

pub fn dos3_call_1000_4fbe(param_1: u8) -> u16 {
    let pc_var1: *mut code;
    let mut c_var2: u8;
    let mut u_var3: u16;
    let mut unaff_BP: i32;

    pc_var1 = swi(0x21);
    unsafe {
        (*pc_var1)(unaff_BP + 1);
    }
    pc_var1 = swi(0x21);
    unsafe {
        c_var2 = (*pc_var1)();
    }
    u_var3 = 0xffff;
    if ((c_var2 + 0x1) == param_1) {
        u_var3 = 0;
    }
    return u_var3;
}

pub fn dos3_call_1000_514e() -> u16 {
    let pc_var1: *mut code;
    let mut u_var2: u16;
    let mut unaff_BP: i32;
    let mut bVar3: bool;

    bVar3 = false;
    pc_var1 = swi(0x21);
    unsafe {
        u_var2 = (*pc_var1)(&g_alloc_addr_1050_1050, unaff_BP + 1);
    }
    if (bVar3) {
        pass1_fn_1000_29b5(u_var2);
        return 0xffff;
    }
    return 0;
}

pub fn dos3_call_1000_5174() -> u32 {
    let pc_var1: *mut code;
    let mut u_var2: u16;
    let mut unaff_BP: i32;
    let mut bVar3: bool;

    bVar3 = false;
    pc_var1 = swi(0x21);
    unsafe {
        u_var2 = (*pc_var1)(unaff_BP + 1);
    }
    if (!bVar3) {
        return 0;
    }
    pass1_fn_1000_29b5(u_var2);
    return u_var2 & 0xff;
}

pub fn dos3_call_1000_51aa(param_1: u16, uparam_2_00: i32, param_2: u16) -> u32 {
    let pc_var1: *mut code;

    pc_var1 = swi(0x21);
    unsafe {
        (*pc_var1)(&g_alloc_addr_1050_1050);
        pc_var1 = swi(0x21);
        (*pc_var1)();
        pc_var1 = swi(0x21);
        (*pc_var1)();
        pc_var1 = swi(0x21);
        (*pc_var1)();
    }
    if ((param_2_00 & 0x100) == 0) {
        return 0;
    }
    pass1_fn_1000_29b5(param_2);
    return param_2 & 0xff;
}

pub fn dos3_call_1000_23ea(a: u16, b: *mut u16) {
    let pbVar1: *mut u8;
    let pb_var2: *mut u8;
    let mut bVar3: u8;
    let mut bVar4: u8;
    let pbVar5: *mut u8;
    let mut i_var6: i32;
    let ppcVar7: fn();
    let pcVar8: *mut code;
    let mut u_var9: u16;
    let mut i_var10: i32;
    let mut unaff_BP: i32;
    let str_142: *mut libc::c_char;
    let pbVar11: *mut u8;
    let pbVar12: *mut u8;
    let unaff_es: *mut u8;
    let mut u_var13: u16;
    let mut bVar14: bool;
    let mut uStack0002: u16;
    let mut fn_ptr_7: u32;

    pcVar8 = swi(0x21);
    unsafe {
        (*pcVar8)(unaff_BP + 1);
    }
    pcVar8 = swi(0x21);
    g_u16_ptr_1050_5f6a = b;
    PTR_LOOP_1050_5f6c = unaff_es;
    unsafe {
        (*pcVar8)();
    }
    u_var13 = 0x1000;
    uStack0002 = u_var13;
    u_var9 = exported_stub_1000_29dc();
    if (&g_fn_ptr_1050_6202 != 0) {
        bVar14 = false;
        ppcVar7 = &g_fn_ptr_1050_6200;
        uStack0002 = u_var13;
        (**ppcVar7)();
        if (bVar14) {
            exit_1000_25cc();
            return;
        }
        fn_ptr_7 = 0x6200;
        uStack0002 = u_var13;
        (**fn_ptr_7)();
    }
    i_var6 = (s_New_failed_in_Op__Op_1050_0020 + 0xc);
    if (i_var6 != 0) {
        pbVar11 = 0x0;
        while {
            unsafe {
                bVar14 = *pbVar11 == 0;
            }
            if (bVar14) {
                break;
            }
            i_var10 = 0xd;
            str_142 = s__C_FILE_INFO__1050_5f5c;
            while {
                if (i_var10 == 0) {
                    break;
                }
                i_var10 = i_var10 + -1;
                pbVar5 = pbVar11;
                pbVar11 = pbVar11 + 1;
                pbVar1 = str_142;
                str_142 = (str_142 + 1);
                unsafe {
                    bVar14 = *pbVar1 == *pbVar5;
                }
                bVar14
            } {}
            if (bVar14) {
                pbVar12 = 0x5f90;
                // goto LAB_1000_2495;
            }
            i_var10 = 0x7fff;
            bVar14 = true;
            while {
                if (i_var10 == 0) {
                    break;
                }
                i_var10 = i_var10 + -1;
                pbVar1 = pbVar11;
                pbVar11 = pbVar11 + 1;
                unsafe {
                    bVar14 = *pbVar1 == 0;
                }
                !bVar14
            } {}
            bVar14
        } {}
    }
    // LAB_1000_24a9:
    uStack0002 = SUB42(s_266_bmp_1050_24ae + 4, 0);
    call_fn_ptr_1000_2594();
    uStack0002 = SUB42(s_264_bmp_1050_24b6 + 5, 0);
    call_fn_ptr_1000_2594();
    uStack0002 = 0x24c4;
    call_fn_ptr_1000_2594();
    return;
    // LAB_1000_2495:
    pb_var2 = pbVar11 + 1;
    unsafe {
        bVar3 = *pbVar11;
    }
    if (bVar3 < 0x41) {}
    // goto LAB_1000_24a9;
    pbVar11 = pbVar11 + 2;
    unsafe {
        bVar4 = *pb_var2;
    }
    if (bVar4 < 0x41) {}
    // goto LAB_1000_24a9;
    pbVar1 = pbVar12;
    pbVar12 = pbVar12 + 1;
    unsafe {
        *pbVar1 = bVar4 + 0xbf | (bVar3 + 0xbf) * '\x10';
    }
    // goto LAB_1000_2495;
}

pub fn dos_api_call_1000_24ff(dos_api_val: u16) {
    let pc_var1: *mut code;
    let mut c_var2: u8;

    *&PTR_LOOP_1050_5fc9 = 1;
    c_var2 = 0x1;
    call_fn_ptr_1000_2594();
    call_fn_ptr_1000_2594();
    call_fn_ptr_1000_256b();
    if (c_var2 == '\0') {
        pc_var1 = swi(0x21);
        unsafe {
            (*pc_var1)();
        }
    }
    return;
}

pub fn reg_class_1008_96d2(param_1: *mut Struct65, param_2: u16) {
    let b_var1: bool;
    let AVar2: ATOM;
    let mut unaff_ss: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    local_6 = param_1 + 0x5b;
    b_var1 = GetClassInfo16(&local_1c, CONCAT22(local_6, unaff_ss), param_1._2_2_);
    if (b_var1 == 0) {
        local_1c = (param_1 + 200);
        local_1a = 0x5632;
        local_18 = &PTR_LOOP_1050_1008;
        local_16 = 0x40000;
        local_12 = g_h_instance_1050_038c;
        local_10 = (param_1 + 0xc2);
        local_e = (param_1 + 0xc4);
        local_c = (param_1 + 0xc6);
        local_a = 0;
        local_4 = param_1._2_2_;
        AVar2 = RegisterClass16(&local_1c);
        if (AVar2 == 0) {
            call_fn_ptr_1000_24cd(0);
        }
    }
    return;
}

pub fn get_sys_metrics_1010_46f6(param_1: u32) {
    let mut u_var1: u16;
    let i_var2: u16;
    let i_var3: u16;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let ppVar6: *mut pass1_struct_1;
    let mut u_var7: u32;
    let puVar8: *mut u16;
    let pu_var9: *mut u8;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pu_var9 = CONCAT22(unaff_ss, &local_4);
    puVar8 = &local_6;
    ppVar6 = process_struct_1010_20ba(_g_Struct372_1050_0ed0, CONCAT22(puVar8, 0x48));
    pass1_1008_3e94((ppVar6 + 0xe), CONCAT22(unaff_ss, puVar8), pu_var9);
    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    u_var7 = process_struct_1008_4772((i_var4 + 0x66));
    u_var1 = (u_var7 >> 0x10);
    (i_var4 + 0x18) = local_4 + 8;
    (i_var4 + 0x1a) = local_6 + 9;
    i_var2 = GetSystemMetrics16(5);
    (i_var4 + 0x1c) = i_var2 * 2 + (u_var7 + 4);
    i_var2 = GetSystemMetrics16(4);
    i_var3 = GetSystemMetrics16(6);
    (i_var4 + 0x1e) = i_var3 + i_var2 + (u_var7 + 8);
    return;
}

pub fn get_sys_metrics_1018_09a8(param_1: u32) {
    let mut u_var1: u32;
    let i_var2: u16;
    let i_var3: u16;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let ppVar6: *mut pass1_struct_1;
    let pu_var7: *mut u16;
    let puVar8: *mut u8;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = GetSystemMetrics16(4);
    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    local_6 = (i_var4 + 0x12) - 2;
    puVar8 = CONCAT22(unaff_ss, &local_8);
    pu_var7 = &local_a;
    ppVar6 = process_struct_1010_20ba(_g_Struct372_1050_0ed0, CONCAT22(pu_var7, 0x48));
    pass1_1008_3e94((ppVar6 + 0xe), CONCAT22(unaff_ss, pu_var7), puVar8);
    (i_var4 + 0x18) = local_6 * local_4 + local_8 + 0x146;
    (i_var4 + 0x1a) = local_6 * local_4 + local_a + 9;
    i_var2 = GetSystemMetrics16(5);
    u_var1 = (i_var4 + 0x5a);
    (i_var4 + 0x1c) = i_var2 * 2 + (u_var1 + 4);
    i_var2 = GetSystemMetrics16(4);
    i_var3 = GetSystemMetrics16(6);
    u_var1 = (i_var4 + 0x5a);
    (i_var4 + 0x1e) = i_var3 + i_var2 + (u_var1 + 8);
    return;
}

pub fn get_sys_metrics_1018_1ea0(param_1: u32) {
    let i_var1: u16;
    let i_var2: u16;
    let mut i_var3: i32;
    let mut u_var4: u16;

    i_var1 = GetSystemMetrics16(5);
    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    (i_var3 + 0x2e) = i_var1 * 2 + (i_var3 + 0x36);
    i_var1 = GetSystemMetrics16(4);
    i_var2 = GetSystemMetrics16(6);
    (i_var3 + 0x30) = i_var1 + (i_var3 + 0x38) + i_var2;
    return;
}

pub fn get_sys_metrics_1018_2f56(param_1: u32) {
    let mut u_var1: u16;
    let i_var2: u16;
    let i_var3: u16;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let ppVar6: *mut pass1_struct_1;
    let mut u_var7: u32;
    let puVar8: *mut u16;
    let pu_var9: *mut u8;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pu_var9 = CONCAT22(unaff_ss, &local_4);
    puVar8 = &local_6;
    ppVar6 = process_struct_1010_20ba(_g_Struct372_1050_0ed0, CONCAT22(puVar8, 0x48));
    pass1_1008_3e94((ppVar6 + 0xe), CONCAT22(unaff_ss, puVar8), pu_var9);
    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    u_var7 = process_struct_1008_4772((i_var4 + 0x24));
    u_var1 = (u_var7 >> 0x10);
    (i_var4 + 0x18) = local_4 + 0xb5;
    (i_var4 + 0x1a) = local_6 + 9;
    i_var2 = GetSystemMetrics16(5);
    (i_var4 + 0x1c) = i_var2 * 2 + (u_var7 + 4);
    i_var2 = GetSystemMetrics16(4);
    i_var3 = GetSystemMetrics16(6);
    (i_var4 + 0x1e) = i_var3 + i_var2 + (u_var7 + 8);
    return;
}

pub fn get_sys_metrics_1018_4b1e(
    param_1: *mut Struct375,
    param_2: u16,
    param_3: u16,
) -> *mut Struct375 {
    let mut i_var1: i32;
    let mut u_var2: u16;

    process_struct_1010_1d48(param_1, param_3);
    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0x12) = param_2;
    (i_var1 + 0x14) = 0;
    param_1.ptr_1_lo = &PTR_LOOP_1050_4c9e;
    (i_var1 + 2) = 0x1018;
    if (PTR_LOOP_1050_416c == 0x0) {
        PTR_LOOP_1050_416c = GetSystemMetrics16(4);
        PTR_LOOP_1050_416e = GetSystemMetrics16(5);
        PTR_LOOP_1050_4170 = GetSystemMetrics16(6);
    }
    return param_1;
}

pub fn call_win_proc_1040_a410(param_1: u16, param_2: u32, param_3: u32) {
    let win_proc: &mut Vec<u8>;
    let mut u_var1: i32;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let mut in_AX: u16;
    let mut in_dx: i32;
    let local_bx_122: *mut Struct130;
    let mut u_var4: u16;
    let mut u_var5: u32;
    let mut local_a: u16;
    let mut local_6: u32;

    local_6 = 0;
    if (param_3 == 0x19) {
        ppc_var2 = (&PTR_LOOP_1050_5ee0 + 0x34);
        local_6 = ppc_var2();
        in_dx = (local_6 >> 0x10);
    } else {
        if (param_3 == 0x86) {
            ppc_var2 = (&PTR_LOOP_1050_5ee0 + 0x20);
            u_var5 = ppc_var2();
            return u_var5;
        }
        if (param_3 == 0x110) {
            u_var3 = &PTR_LOOP_1050_5ee0;
            u_var5 = send_win_msg_1040_a308(u_var3, (u_var3 >> 0x10), param_1, param_2);
            return u_var5;
        }
    }
    if (local_6 != 0) {
        return local_6 & 0xffff | in_dx << 0x10;
    }
    u_var3 = &PTR_LOOP_1050_5bc8;
    u_var4 = (u_var3 >> 0x10);
    local_bx_122 = u_var3;
    win_proc = local_bx_122.fn_ptr_0x4;
    u_var1 = local_bx_122.field_0x6;
    if ((u_var1 | win_proc) == 0) {
        return u_var1 << 0x10;
    }
    u_var5 = CallWindowProc16(
        CONCAT22(param_2, param_1),
        (param_2 >> 0x10),
        param_3,
        (param_3 >> 0x10),
        win_proc,
    );
    return u_var5;
}

pub fn send_win_msg_1040_a3d0(param_1: u32) {
    let mut u_var1: u32;
    let HVar2: HWND16;
    let mut i_var3: i32;
    let mut u_var4: u16;

    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    if ((i_var3 + 0x90) != 0) {
        u_var1 = (i_var3 + 0x90);
        (u_var1 + 0x14) = (i_var3 + 6);
        HVar2 = GetDlgItem16(0x1826, (i_var3 + 6));
        send_win_msg_1040_a308(param_1, 0, HVar2);
    }
    return;
}

pub fn send_win_msg_1040_a308(param_1: u32, param_2: u16, param_3: u16, param_4: HWND16) {
    let pu_var1: *mut u16;
    let mut u_var2: u32;
    let LVar3: LPARAM;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let ppVar6: *mut pass1_struct_1;
    let WVar7: WPARAM16;
    let mut u_var8: u16;
    let HVar9: HWND16;
    let mut in_stack_0000fff2: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    SendMessage16(0, 0, 0x405, param_2);
    SendMessage16(0, 0, 0xb, param_2);
    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    u_var2 = (i_var4 + 0x90);
    if ((u_var2 + 0x10) == 0) {
        WVar7 = 0;
        u_var8 = 0x401;
        HVar9 = param_2;
        LVar3 = load_string_1010_847e(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            0x531,
        );
        SendMessage16(LVar3, WVar7, u_var8, HVar9);
    } else {
        ppVar6 = process_struct_1010_20ba(_g_Struct372_1050_0ed0, CONCAT22(in_stack_0000fff2, 3));
        local_c = 0;
        while (
            u_var2 = (i_var4 + 0x90),
            pu_var1 = (u_var2 + 0x10),
            unsafe { *pu_var1 != local_c && local_c <= *pu_var1 },
        ) {
            WVar7 = 0;
            u_var8 = 0x401;
            u_var2 = (i_var4 + 0x90);
            u_var2 = (u_var2 + 0xc);
            HVar9 = param_2;
            LVar3 = pass1_1010_ac92(ppVar6, (ppVar6 >> 0x10), (u_var2 + local_c * 2));
            SendMessage16(LVar3, WVar7, u_var8, HVar9);
            local_c = local_c + 1;
        }
    }
    SendMessage16(0, 1, 0xb, param_2);
    return 1;
}

pub fn win_fn_1020_0ec4(param_1: *mut u32, uparam_2: i32) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut cVar3: u8;
    let mut i_var4: i32;
    let mut i_var5: i32;
    let mut u_var6: i32;
    let mut extraout_dx: u16;
    let mut extraout_dx_00: u16;
    let mut extraout_dx_01: u16;
    let mut u_var7: u16;
    let pp_var8: *mut pass1_struct_1;
    let w_param: WPARAM16;
    let h_wnd: HWND16;
    let mut u_var9: u16;
    let mut local_c: u16;
    let mut local_a: u16;

    u_var7 = (param_1 >> 0x10);
    i_var4 = param_1;
    if (param_2 == 0xfb) {
        pp_var8 = process_struct_1010_20ba(_g_Struct372_1050_0ed0, CONCAT22(local_c, 0x30));
        u_var9 = SUB42(pp_var8, 0);
        pass1_1010_375e(pp_var8);
        unsafe {
            ppc_var2 = (*param_1 + 0x14);
        }
        ppc_var2(0x1010, param_1, u_var7, 1, u_var9, extraout_dx);
        pass1_1010_375e(pp_var8);
        pass1_1010_4788((i_var4 + 0xf2), CONCAT22(extraout_dx_00, u_var9));
        return;
    }
    if (0xfb < param_2) {
        match (param_2) {
            _ => {
                return;
            }
            0x12a => {
                h_wnd = (i_var4 + 8);
                w_param = 0xf012;
            }
            300 => {
                h_wnd = (i_var4 + 8);
                w_param = 0xf020;
            }
        }
        PostMessage16(0, w_param, 0x112, h_wnd);
        return;
    }
    if (param_2 == 0xf3) {
        u_var9 = 3;
    } else {
        if (0xf3 < param_2) {
            return;
        }
        cVar3 = param_2;
        u_var6 = param_2 & 0xff00 | (cVar3 + 0x91);
        if ((cVar3 + 0x91) == 0) {
            mixed_fn_1010_830a(_g_struct_73_1050_14cc, 0x1f8);
            WinHelp16(0x28, 1, CONCAT22(extraout_dx_01, u_var6), (i_var4 + 8));
            return;
        }
        if (cVar3 == 'r') {
            i_var5 = i_var4 + 10;
            u_var9 = u_var7;
            pp_var8 = process_struct_1010_20ba(_g_Struct372_1050_0ed0, CONCAT22(i_var5, 0x30));
            pass1_1010_3770(pp_var8, CONCAT22(u_var9, i_var5));
            pass1_1038_af40(_g_Struct112_a, *(i_var4 + 8), 3);
            return;
        }
        if (cVar3 == -0xf) {
            u_var9 = 1;
        } else {
            if (cVar3 != -0xe) {
                return;
            }
            u_var9 = 2;
        }
    }
    u_var1 = (i_var4 + 0xf2);
    pass1_1010_4674(u_var1, (u_var1 >> 0x10), u_var9);
    return;
}

pub fn call_win_fn_1020_0e8e(in_struct_1: *mut Struct637, param_2: u32, param_3: u32) {
    let mut i_var1: i32;
    // fn_ptr_1: *mut *mut u8;

    i_var1 = win_fn_1020_1294(CONCAT22(param_2, in_struct_1), (param_2 >> 0x10), param_3);
    if (i_var1 == 0) {
        fn_ptr_1 = (in_struct_1.field_0x4 + 0x5c);
        (**fn_ptr_1)();
    }
    return;
}

// WARNING: Variable defined which should be unmapped: local_c
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn win_msg_fn_1020_0ae8(in_struct_1: *mut Struct376, param_2: u8) -> *mut Struct376 {
    send_win_msg_1020_08fe(in_struct_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(in_struct_1);
    }
    return in_struct_1;
}

pub fn send_win_msg_1020_08fe(in_struct_1: *mut Struct376) {
    let hwnd: HWND16;
    let b_var1: bool;
    let local_struct_1: *mut Struct376;
    let local_struct_1_hi: *mut Struct376;
    let mut local_4: u16;
    let mut temp_5f73679df1: u32;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    in_struct_1.ptr_a_lo = 0xb0e;
    local_struct_1.ptr_a_hi = 0x1020;
    if (&local_struct_1.field_0xe8 != 0) {
        temp_5f73679df1 = &local_struct_1.field_0xe8;
        hwnd = (temp_5f73679df1 + 6);
        b_var1 = IsWindow16(hwnd);
        if (b_var1 != 0) {
            SendMessage16(0, 1, 0x111, hwnd);
        }
        &local_struct_1.field_0xe8 = 0;
    }
    pass1_1008_57c4((in_struct_1 & 0xffff0000 | ZEXT24(&local_struct_1.struct_215_ptr_0xd2)));
    in_struct_1.ptr_a_lo = 0x380a;
    local_struct_1.ptr_a_hi = &PTR_LOOP_1050_1008;
    in_struct_1.ptr_a_lo = s_1_1050_389a;
    local_struct_1.ptr_a_hi = &PTR_LOOP_1050_1008;
    return;
}

pub fn send_win_msg_1020_097e(param_1: u32) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut u_var3: u16;

    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    if (((i_var2 + 0xea) | (i_var2 + 0xe8)) != 0) {
        u_var1 = (i_var2 + 0xe8);
        SendMessage16(0, 1, 0x111, (u_var1 + 6));
        (i_var2 + 0xe8) = 0;
    }
    return;
}

pub fn post_msg_1020_03b2(param_1: u32) {
    let mut u_var1: u32;

    u_var1 = (param_1 + 0xe2);
    PostMessage16(0, (u_var1 + 0x16), 0x111, g_h_window);
    return;
}

pub fn post_msg_1020_03d6(param_1: u32) {
    let mut u_var1: u32;

    u_var1 = (param_1 + 0xe2);
    PostMessage16(0, (u_var1 + 0x16), 0x111, g_h_window);
    return;
}

pub fn post_msg_1020_03fa(param_1: u32) {
    let mut u_var1: u32;

    u_var1 = (param_1 + 0xe2);
    PostMessage16(0, (u_var1 + 0x16), 0x111, g_h_window);
    return;
}

pub fn post_win_msg_1020_061c(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    if (param_2 == 1) {
        (param_1 + 6) = 0;
        return;
    }
    if (param_2 != 2) {
        return;
    }
    u_var1 = (param_1 + 6);
    PostMessage16(0, (u_var1 + 0x16), 0x111, g_h_window);
    return;
}

pub fn win_func_1018_6bb6(param_1: u32) {
    let b_var1: bool;
    let mut i_var2: i32;
    let mut u_var3: u16;

    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    if ((i_var2 + 0xea) != 0) {
        PostMessage16(0, (i_var2 + 0xea), 0x111, g_h_window);
    }
    PostMessage16(0, 0x79, 0x111, g_h_window);
    if ((i_var2 + 0xf0) != 0) {
        b_var1 = IsWindow16((i_var2 + 0xf0));
        if (b_var1 != 0) {
            DestroyWindow16((i_var2 + 0xf0));
            (i_var2 + 0xf0) = 0;
        }
    }
    return;
}

pub fn send_win_msg_1010_7c9e(param_1: u32, param_2: u16) {
    let mut u_var1: u32;
    let BVar2: bool;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let lVar5: u32;
    let mut u_var6: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: [u8; 8];

    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    if ((((i_var3 + 0x1e) | (i_var3 + 0x1c)) != 0) && (param_2 != 0)) {
        pass1_1008_5784(CONCAT22(unaff_ss, local_a), (i_var3 + 0x1c));
        while (true) {
            lVar5 = pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
            u_var4 = (lVar5 >> 0x10);
            i_var3 = lVar5;
            if (lVar5 == 0) {
                break;
            }
            if ((i_var3 + 4) != 0) {
                u_var6 = pass1_1030_73a8((i_var3 + 4));
                BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (u_var6 + 0xc), param_2);
                if (BVar2 != 0) {
                    u_var1 = (i_var3 + 8);
                    SendMessage16(0, 0xeb, 0x111, (u_var1 + 6));
                }
            }
        }
    }
    return;
}

pub fn send_msg_1010_7c42(param_1: u32) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut unaff_ss: u16;
    let lVar4: u32;
    let mut local_e: u32;
    let mut local_a: [u8; 8];

    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    if (((i_var2 + 0x1e) | (i_var2 + 0x1c)) != 0) {
        pass1_1008_5784(CONCAT22(unaff_ss, local_a), (i_var2 + 0x1c));
        while (true) {
            lVar4 = pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
            if (lVar4 == 0) {
                break;
            }
            u_var1 = (lVar4 + 8);
            SendMessage16(0, 0xeb, 0x111, (u_var1 + 6));
        }
    }
    return;
}

pub fn write_private_profile_str_1010_5b10(param_1: *mut Struct376) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let mut u_var3: u32;
    let ppcVar4: fn();
    let mut u_var5: u16;
    let local_bx_5: *mut Struct77;
    let mut unaff_si: u16;
    let mut u_var6: u16;
    let pp_var7: *mut pass1_struct_1;
    let mut local_c: u16;
    let mut local_8: u16;

    u_var6 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    param_1.ptr_a_lo = 0x6312;
    local_bx_5.field_0x2 = 0x1010;
    pp_var7 = process_struct_1010_20ba(_g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x48));
    u_var3 = local_bx_5.field_0xe;
    string_fn_1000_3f9c(
        u_var3,
        (u_var3 >> 0x10),
        s__d__d_1050_149c,
        &g_alloc_addr_1050_1050,
        *(pp_var7 + 10),
    );
    if (local_bx_5.field_0x80 == 0) {
        u_var5 = SUB42(s_off_1050_13c8, 0);
    } else {
        u_var5 = SUB42(s_on_1050_13c4, 0);
    }
    WritePrivateProfileString(
        local_bx_5.field_0xa,
        CONCAT22(0x1050, u_var5),
        s_falseMap_1050_13de,
        s_general_1050_13b0,
    );
    WritePrivateProfileString(
        local_bx_5.field_0xa,
        local_bx_5.field_0xe,
        s_rez_1050_13c0,
        s_general_1050_13b0,
    );
    if (local_bx_5.field_0x1e == 0) {
        u_var5 = SUB42(s_off_1050_13c8, 0);
    } else {
        u_var5 = SUB42(s_on_1050_13c4, 0);
    }
    WritePrivateProfileString(
        local_bx_5.field_0xa,
        CONCAT22(0x1050, u_var5),
        s_anims_1050_13cc,
        s_general_1050_13b0,
    );
    if (local_bx_5.field_0x74 == 0) {
        u_var5 = SUB42(s_off_1050_13c8, 0);
    } else {
        u_var5 = SUB42(s_on_1050_13c4, 0);
    }
    WritePrivateProfileString(
        local_bx_5.field_0xa,
        CONCAT22(0x1050, u_var5),
        s_music_1050_13d2,
        s_general_1050_13b0,
    );
    if (local_bx_5.field_0x72 == 0) {
        u_var5 = SUB42(s_off_1050_13c8, 0);
    } else {
        u_var5 = SUB42(s_on_1050_13c4, 0);
    }
    WritePrivateProfileString(
        local_bx_5.field_0xa,
        CONCAT22(0x1050, u_var5),
        s_sound_1050_13d8,
        s_general_1050_13b0,
    );
    if (local_bx_5.field_0x20 == 0) {
        u_var5 = SUB42(s_off_1050_13c8, 0);
    } else {
        u_var5 = SUB42(s_on_1050_13c4, 0);
    }
    WritePrivateProfileString(
        local_bx_5.field_0xa,
        CONCAT22(0x1050, u_var5),
        s_movies_1050_13e8,
        s_general_1050_13b0,
    );
    u_var3 = local_bx_5.field_0xe;
    string_fn_1000_3f9c(
        u_var3,
        (u_var3 >> 0x10),
        s__lu_1050_14a2,
        &g_alloc_addr_1050_1050,
        local_bx_5.field_0x76,
    );
    WritePrivateProfileString(
        local_bx_5.field_0xa,
        local_bx_5.field_0xe,
        s_turns_1050_1466,
        s_general_1050_13b0,
    );
    if (local_bx_5.field_0x7a == 0) {
        u_var5 = SUB42(s_off_1050_13c8, 0);
    } else {
        u_var5 = SUB42(s_on_1050_13c4, 0);
    }
    WritePrivateProfileString(
        local_bx_5.field_0xa,
        CONCAT22(0x1050, u_var5),
        s_turnsDlgStatus_1050_146c,
        s_general_1050_13b0,
    );
    WritePrivateProfileString(
        local_bx_5.field_0xa,
        local_bx_5.field_0x1a,
        s_savePath_1050_147c,
        s_general_1050_13b0,
    );
    WritePrivateProfileString(
        local_bx_5.field_0xa,
        local_bx_5.field_0x68,
        s_aiName_1050_1486,
        s_general_1050_13b0,
    );
    WritePrivateProfileString(
        local_bx_5.field_0xa,
        local_bx_5.field_0x6c,
        s_playerName_1050_148e,
        s_general_1050_13b0,
    );
    local_c = 1;
    while {
        // switchD_1010:2ab5:
        write_private_profile_str_1010_622a(local_bx_5, u_var6, local_c);
        local_c = local_c + 1;
        local_c < 8
    } {}
    error_check_1000_17ce(local_bx_5.field_0xa);
    error_check_1000_17ce(local_bx_5.field_0xe);
    error_check_1000_17ce(local_bx_5.field_0x12);
    error_check_1000_17ce(local_bx_5.field_0x16);
    error_check_1000_17ce(local_bx_5.field_0x1a);
    pu_var1 = local_bx_5.field_0x64;
    u_var2 = local_bx_5.field_0x66;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            ppcVar4 = *pu_var1;
        }
        (**ppcVar4)(0x1000, pu_var1, u_var2, 1);
    }
    error_check_1000_17ce(local_bx_5.field_0x68);
    error_check_1000_17ce(local_bx_5.field_0x6c);
    pass1_1010_1d80(param_1);
    return;
}

pub fn get_private_profile_str_1010_5404(param_1: *mut Struct64, param_2: u32) {
    let pi_var1: *mut i32;
    let pu_var2: *mut u32;
    let mut u_var3: u32;
    let ppcVar4: fn();
    let mut u_var5: u32;
    let u_var6: u8;
    let mut u_var7: i32;
    let mut u_var8: u16;
    let mut i_var9: i32;
    let i_var10: u16;
    let pa_var11: *mut Struct199;
    let mut u_var12: i32;
    let extraout_var: u32;
    let struct_a: *mut Struct199;
    let extraout_dx: *mut Struct199;
    let extraout_dx_00: *mut Struct199;
    let paVar14: *mut Struct199;
    let mut extraout_dx_01: i32;
    let mut extraout_dx_02: i32;
    let pu_var15: *mut u8;
    let mut uVar16: u16;
    let mut unaff_ss: u16;
    let ppVar17: *mut pass1_struct_1;
    let uVar18: u8;
    let uVar19: u8;
    let mut u_var20: u16;
    let mut u_var21: u16;
    let mut u_var22: u16;
    let mut u_var23: u16;
    let local_135: u8;
    let local_134: u8;
    let local_133: u8;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let pu_var13: *mut u16;

    u_var22 = param_2;
    process_struct_1010_1d48(CONCAT22(u_var22, param_1), (param_2 >> 0x10));
    u_var8 = 0;
    &param_1.field_0xa = 0;
    &param_1.field_0xe = 0;
    param_1.field_0x12 = 0;
    param_1.field_0x16 = 0;
    &param_1.field_0x1a = 0;
    param_1.field_0x62 = 0;
    &param_1.field_0x64 = 0;
    &param_1.field_0x68 = 0;
    &param_1.field_0x6c = 0;
    param_1.field_0x70 = 1;
    param_1.field_0x7a = 0;
    param_1.field_0x7c = 0;
    param_1.field_0x7e = 0;
    param_1.field_0x80 = 0;
    param_1.field_0x82 = 1;
    CONCAT22(u_var22, param_1) = 0x6312;
    param_1.field_0x2 = 0x1010;
    str_fn_1010_6034(CONCAT22(u_var22, param_1));
    paVar14 = struct_a;
    process_struct_1000_179c(0x101, struct_a);
    param_1.field_0xe = u_var8;
    &param_1.field_0x10 = paVar14;
    pass1_fn_1000_5008(param_1.field_0xe, paVar14, 0x100);
    u_var7 = get_string_index_1000_3da4(*&param_1.field_0xe);
    u_var5 = &param_1.field_0xe;
    uVar16 = (u_var5 >> 0x10);
    pu_var15 = (u_var5 + u_var7);
    if (pu_var15[-1] != '\\') {
        unsafe {
            *pu_var15 = 0x5c;
        }
        u_var5 = &param_1.field_0xe;
        *(u_var5 + u_var7 + 1) = 0;
    }
    local_4 = u_var7;
    load_string_1010_847e(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x578,
    );
    paVar14 = extraout_dx;
    local_8 = u_var7;
    local_6 = extraout_dx;
    process_string_1000_3cea(*&param_1.field_0xe, CONCAT22(extraout_dx, u_var7));
    u_var5 = &param_1.field_0xe;
    pass1_fn_1008_60e8(u_var5, (u_var5 >> 0x10));
    param_1.field_0xa = u_var7;
    &param_1.field_0xc = paVar14;
    u_var23 = SUB42(&g_alloc_addr_1050_1050, 0);
    u_var5 = &param_1.field_0xe;
    uVar16 = SUB42(offset, 0);
    u_var8 = GetPrivateProfileString16(
        CONCAT22(param_1.field_0xa, 0x1008),
        paVar14,
        CONCAT22(u_var5, 0x100),
        CONCAT22(0x149a, (u_var5 >> 0x10)),
        0x13c01050,
        0x13b01050,
    );
    if (**&param_1.field_0xe != '\0') {
        uVar16 = 0x1000;
        pass1_fn_1000_3e2c(&param_1.field_0xe);
        local_16 = u_var8;
        ppVar17 = process_struct_1010_20ba(_g_Struct372_1050_0ed0, CONCAT22(u_var23, 0x48));
        paVar14 = (ppVar17 >> 0x10);
        local_1a = ppVar17;
        local_a = (local_1a + 10);
        local_c = (local_1a + 0xc);
        param_1.field_0x62 = (local_16 != local_a);
        local_18 = paVar14;
    }
    u_var5 = &param_1.field_0xe;
    u_var3 = &param_1.field_0xa;
    u_var23 = SUB42(offset, 0);
    GetPrivateProfileString16(
        CONCAT22(u_var3, uVar16),
        (u_var3 >> 0x10),
        CONCAT22(u_var5, 0x100),
        CONCAT22(0x149a, (u_var5 >> 0x10)),
        0x13de1050,
        0x13b01050,
    );
    if (**&param_1.field_0xe != '\0') {
        u_var23 = 0x1000;
        i_var9 = process_string_1000_475e(*&param_1.field_0xe, s_on_1050_13c4);
        if (i_var9 == 0) {
            param_1.field_0x80 = 1;
        }
    }
    u_var5 = &param_1.field_0xe;
    u_var3 = &param_1.field_0xa;
    uVar16 = SUB42(offset, 0);
    GetPrivateProfileString16(
        CONCAT13((u_var3 >> 8), CONCAT12(u_var3, u_var23)),
        (u_var3 >> 0x10),
        CONCAT22(u_var5, 0x100),
        CONCAT22(0x149a, (u_var5 >> 0x10)),
        0x13d21050,
        0x13b01050,
    );
    if (**&param_1.field_0xe != '\0') {
        uVar16 = 0x1000;
        i_var9 = process_string_1000_475e(*&param_1.field_0xe, s_off_1050_13c8);
        if (i_var9 == 0) {
            param_1.field_0x74 = 0;
        }
    }
    u_var5 = &param_1.field_0xe;
    u_var3 = &param_1.field_0xa;
    u_var23 = SUB42(offset, 0);
    GetPrivateProfileString16(
        CONCAT13((u_var3 >> 8), CONCAT12(u_var3, uVar16)),
        (u_var3 >> 0x10),
        CONCAT22(u_var5, 0x100),
        CONCAT22(0x149a, (u_var5 >> 0x10)),
        0x13d81050,
        0x13b01050,
    );
    if (**&param_1.field_0xe != '\0') {
        u_var23 = 0x1000;
        i_var9 = process_string_1000_475e(*&param_1.field_0xe, s_off_1050_13c8);
        if (i_var9 == 0) {
            param_1.field_0x72 = 0;
        }
    }
    u_var5 = &param_1.field_0xe;
    u_var3 = &param_1.field_0xa;
    uVar16 = SUB42(offset, 0);
    GetPrivateProfileString16(
        CONCAT13((u_var3 >> 8), CONCAT12(u_var3, u_var23)),
        (u_var3 >> 0x10),
        CONCAT22(u_var5, 0x100),
        CONCAT22(0x149a, (u_var5 >> 0x10)),
        0x13cc1050,
        0x13b01050,
    );
    if (**&param_1.field_0xe != '\0') {
        uVar16 = 0x1000;
        i_var9 = process_string_1000_475e(*&param_1.field_0xe, s_off_1050_13c8);
        if (i_var9 == 0) {
            param_1.field_0x1e = 0;
        }
    }
    u_var5 = &param_1.field_0xe;
    u_var3 = &param_1.field_0xa;
    u_var23 = SUB42(offset, 0);
    GetPrivateProfileString16(
        CONCAT13((u_var3 >> 8), CONCAT12(u_var3, uVar16)),
        (u_var3 >> 0x10),
        CONCAT13((u_var5 >> 8), CONCAT12(u_var5, 0x100)),
        CONCAT22(0x149a, (u_var5 >> 0x10)),
        0x13e81050,
        0x13b01050,
    );
    if (**&param_1.field_0xe != '\0') {
        u_var23 = 0x1000;
        i_var9 = process_string_1000_475e(*&param_1.field_0xe, s_off_1050_13c8);
        if (i_var9 == 0) {
            param_1.field_0x20 = 0;
        }
    }
    u_var5 = &param_1.field_0xe;
    u_var3 = &param_1.field_0xa;
    uVar16 = SUB42(offset, 0);
    u_var7 = GetPrivateProfileString16(
        CONCAT22(u_var3, u_var23),
        (u_var3 >> 0x10),
        CONCAT13((u_var5 >> 8), CONCAT12(u_var5, 0x100)),
        CONCAT22(0x149a, (u_var5 >> 0x10)),
        0x14661050,
        0x13b01050,
    );
    pa_var11 = paVar14;
    if (**&param_1.field_0xe != '\0') {
        uVar16 = 0x1000;
        pass1_fn_1000_3e2c(&param_1.field_0xe);
        pa_var11 = (paVar14 | u_var7);
        local_2e = u_var7;
        local_2c = paVar14;
        if ((paVar14 | u_var7) != 0x0) {
            param_1.field_0x76 = u_var7;
            param_1.field_0x78 = paVar14;
            pa_var11 = paVar14;
        }
    }
    u_var21 = SUB42(&g_alloc_addr_1050_1050, 0);
    u_var5 = &param_1.field_0xe;
    u_var3 = &param_1.field_0xa;
    u_var23 = SUB42(offset, 0);
    GetPrivateProfileString16(
        CONCAT22(u_var3, uVar16),
        (u_var3 >> 0x10),
        CONCAT13((u_var5 >> 8), CONCAT12(u_var5, 0x100)),
        CONCAT13(0x14, CONCAT12(0x9a, (u_var5 >> 0x10))),
        0x146c1050,
        0x13b01050,
    );
    if (**&param_1.field_0xe != '\0') {
        u_var23 = 0x1000;
        i_var9 = process_string_1000_475e(*&param_1.field_0xe, s_on_1050_13c4);
        if (i_var9 == 0) {
            param_1.field_0x7a = 1;
        }
    }
    u_var20 = SUB42(&g_alloc_addr_1050_1050, 0);
    u_var5 = &param_1.field_0xe;
    u_var3 = &param_1.field_0xa;
    uVar16 = SUB42(offset, 0);
    i_var10 = GetPrivateProfileString16(
        CONCAT13((u_var3 >> 8), CONCAT12(u_var3, u_var23)),
        (u_var3 >> 0x10),
        CONCAT13((u_var5 >> 8), CONCAT12(u_var5, 0x100)),
        CONCAT13(0x14, CONCAT12(0x9a, (u_var5 >> 0x10))),
        0x147c1050,
        0x13b01050,
    );
    if (**&param_1.field_0xe != '\0') {
        u_var5 = &param_1.field_0xe;
        uVar16 = SUB42(&PTR_LOOP_1050_1008, 0);
        pass1_fn_1008_60e8(u_var5, (u_var5 >> 0x10), u_var20, u_var21);
        param_1.field_0x1a = i_var10;
        param_1.field_0x1c = pa_var11;
    }
    u_var21 = SUB42(&g_alloc_addr_1050_1050, 0);
    u_var5 = &param_1.field_0xe;
    u_var3 = &param_1.field_0xa;
    u_var23 = SUB42(offset, 0);
    i_var10 = GetPrivateProfileString16(
        CONCAT22(u_var3, uVar16),
        (u_var3 >> 0x10),
        CONCAT22(u_var5, 0x100),
        CONCAT13(0x14, CONCAT12(0x9a, (u_var5 >> 0x10))),
        0x14861050,
        0x13b01050,
    );
    if (**&param_1.field_0xe != '\0') {
        u_var5 = &param_1.field_0xe;
        u_var23 = SUB42(&PTR_LOOP_1050_1008, 0);
        pass1_fn_1008_60e8(u_var5, (u_var5 >> 0x10), u_var21);
        param_1.field_0x68 = i_var10;
        param_1.field_0x6a = pa_var11;
    }
    uVar16 = SUB42(&g_alloc_addr_1050_1050, 0);
    u_var5 = &param_1.field_0xe;
    u_var3 = &param_1.field_0xa;
    u_var7 = GetPrivateProfileString16(
        CONCAT13((u_var3 >> 8), CONCAT12(u_var3, u_var23)),
        (u_var3 >> 0x10),
        CONCAT22(u_var5, 0x100),
        CONCAT13(0x14, CONCAT12(0x9a, (u_var5 >> 0x10))),
        0x148e1050,
        0x13b01050,
    );
    if (**&param_1.field_0xe != '\0') {
        u_var5 = &param_1.field_0xe;
        pass1_fn_1008_60e8(u_var5, (u_var5 >> 0x10), uVar16);
        param_1.field_0x6c = u_var7;
        param_1.field_0x6e = pa_var11;
    }
    if (param_1.field_0x62 == 0) {
        local_2e = GetSystemMetrics16(4);
        local_2a = 1;
        while {
            get_private_profile_str_1010_6132(param_1, u_var22, local_2a);
            i_var9 = &param_1.field_0x0 + local_2a * 8;
            pa_var11 = extraout_dx_00;
            let pi_var1_val = unsafe { *pi_var1 };
            let pu_var2_val = unsafe { *pu_var2 };

            if (((((i_var9 + 0x22) < 0) || ((i_var9 + 0x24) < 0))
                || (
                    pi_var1 = (i_var9 + 0x22),
                    pi_var1_val != local_a - local_2e && (local_a - local_2e) <= pi_var1_val,
                ))
                || (
                    u_var7 = local_c - local_2e,
                    pu_var2 = (i_var9 + 0x24),
                    pu_var2_val != u_var7 && u_var7 <= pu_var2_val,
                ))
            {
                u_var6 = pass1_1000_4906(
                    CONCAT13(
                        (param_2 >> 8),
                        CONCAT12(param_2, &param_1.field_0x22 + local_2a * 8),
                    ),
                    0,
                    8,
                );
                u_var7 = CONCAT31(extraout_var, u_var6);
            }
            local_2a = local_2a + 1;
            local_2a < 8
        } {}
    }
    process_struct_1000_179c(0xc, pa_var11);
    _local_32 = CONCAT22(pa_var11, u_var7);
    paVar14 = (pa_var11 | u_var7);
    if (paVar14 == 0x0) {
        pa_var11 = 0x0;
        paVar14 = 0x0;
    } else {
        pa_var11 = process_struct_1008_574a(CONCAT22(pa_var11, u_var7));
    }
    param_1.field_0x64 = pa_var11;
    &param_1.field_0x66 = paVar14;
    u_var5 = &param_1.field_0xe;
    pass1_fn_1000_5008(u_var5, (u_var5 >> 0x10), 0x100);
    u_var7 = get_string_index_1000_3da4(*&param_1.field_0xe);
    u_var5 = &param_1.field_0xe;
    uVar16 = (u_var5 >> 0x10);
    pu_var15 = (u_var5 + u_var7);
    if (pu_var15[-1] != '\\') {
        unsafe {
            *pu_var15 = 0x5c;
        }
        u_var5 = &param_1.field_0xe;
        *(u_var5 + u_var7 + 1) = 0;
    }
    u_var5 = &param_1.field_0xe;
    local_4 = u_var7;
    pass1_fn_1008_60e8(u_var5, (u_var5 >> 0x10));
    _local_10 = CONCAT22(paVar14, u_var7);
    process_struct_1000_179c(8, paVar14);
    _local_32 = CONCAT22(paVar14, u_var7);
    if ((paVar14 | u_var7) == 0) {
        local_14 = 0;
    } else {
        *_local_32 = s_1_1050_389a;
        (u_var7 + 2) = &PTR_LOOP_1050_1008;
        (u_var7 + 4) = _local_10;
        *_local_32 = 0x6322;
        (u_var7 + 2) = 0x1010;
        local_14 = _local_32;
    }
    u_var5 = &param_1.field_0x64;
    ppcVar4 = (&param_1.field_0x64 + 4);
    (**ppcVar4)(0, u_var5, (u_var5 >> 0x10), local_14, (local_14 >> 0x10));
    u_var5 = &param_1.field_0xe;
    u_var3 = &param_1.field_0xa;
    u_var7 = extraout_dx_01;
    u_var12 = GetPrivateProfileString16(
        CONCAT22(u_var3, 0x1000),
        (u_var3 >> 0x10),
        CONCAT22(u_var5, 0x100),
        CONCAT22(0x149a, (u_var5 >> 0x10)),
        0x13f01050,
        0x13b01050,
    );
    pu_var13 = u_var12;
    if (**&param_1.field_0xe != '\0') {
        u_var5 = &param_1.field_0xe;
        uVar18 = u_var5;
        uVar19 = (u_var5 >> 8);
        uVar16 = (u_var5 >> 0x10);
        while (true) {
            u_var12 = pu_var13;
            pass1_fn_1000_47a4(CONCAT22(uVar16, CONCAT11(uVar19, uVar18)), s___1050_13f8);
            if ((u_var7 | u_var12) == 0) {
                break;
            }
            local_2e = u_var12;
            local_2c = u_var7;
            paVar14 = copy_string_1000_3d3e(
                CONCAT22(unaff_ss, &local_134),
                CONCAT13((u_var7 >> 8), CONCAT12(u_var7, u_var12)),
            );
            local_4 = get_string_index_1000_3da4(CONCAT13(
                (unaff_ss >> 8),
                CONCAT12(unaff_ss, &local_134),
            ));
            if ((&local_135)[local_4] != '\\') {
                (&local_134)[local_4] = 0x5c;
                (&local_133)[local_4] = '\0';
            }
            pu_var13 = ZEXT24(&local_134);
            pass1_fn_1008_60e8(&local_134);
            _local_10 = pu_var13 & 0xffff | ZEXT24(paVar14) << 0x10;
            process_struct_1000_179c(8, paVar14);
            u_var7 = pu_var13;
            _local_32 = (pu_var13 & 0xffff | ZEXT24(paVar14) << 0x10);
            if ((paVar14 | u_var7) == 0) {
                local_14 = 0;
            } else {
                *_local_32 = s_1_1050_389a;
                (u_var7 + 2) = &PTR_LOOP_1050_1008;
                (u_var7 + 4) = _local_10;
                *_local_32 = 0x6322;
                (u_var7 + 2) = 0x1010;
                pu_var13 = _local_32;
                local_14 = _local_32;
            }
            u_var5 = &param_1.field_0x64;
            ppcVar4 = (&param_1.field_0x64 + 8);
            (**ppcVar4)(
                0x1000,
                u_var5,
                (u_var5 >> 0x10),
                local_14,
                (local_14 >> 0x10),
            );
            uVar18 = 0;
            uVar19 = 0;
            uVar16 = 0;
            u_var7 = extraout_dx_02;
        }
    }
    return;
}

pub fn win_cleanup_func_1040_b0f8(param_1: *mut Struct44) -> u8 {
    let mut u_var1: i32;
    let mut u_var2: i32;
    let u_var3: u8;
    let local_bx_4: *mut Struct348;
    let mut u_var4: u16;
    let ppVar5: *mut pass1_struct_1;
    let mut in_stack_0000ffe8: u32;
    let mut local_a: u16;
    let mut local_8: u16;

    u_var4 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    param_1.ptr_a_lo = 0xb772;
    local_bx_4.field_0x2 = &PTR_LOOP_1050_1040;
    ppVar5 = process_struct_1010_20ba(
        _g_Struct372_1050_0ed0,
        CONCAT22((in_stack_0000ffe8 >> 0x10), 0x32),
    );
    pass1_1010_7b8c(ppVar5, local_bx_4.field_0x6);
    if (local_bx_4[0x11].field_0x6 != 0) {
        DeleteObject16(local_bx_4[0x11].field_0x6);
        local_bx_4[0x11].field_0x6 = 0;
    }
    u_var1 = (local_bx_4 + 0x12).field_0x0;
    u_var2 = local_bx_4[0x12].field_0x2;
    _local_a = CONCAT22(u_var2, u_var1);
    if ((u_var2 | u_var1) != 0) {
        pass1_1040_a5d0(CONCAT22(u_var2, u_var1));
        error_check_1000_17ce(_local_a);
    }
    u_var3 = win_cleanup_func_1040_782c(param_1);
    return u_var3;
}

pub fn win_fn_1010_0242(param_1: u16, param_2: u16, param_3: HWND16) -> u16 {
    let pp_var1: fn();
    let mut in_AX: u16;
    let BVar2: bool;
    let wVar3: u16;
    let ppVar4: *mut pass1_struct_1;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    BVar2 = IsWindow16(param_1);
    if (BVar2 != 0) {
        wVar3 = GetWindowWord16(-6, param_1);
        if (wVar3 == &g_h_instance_1050_038c) {
            BVar2 = IsIconic16(param_1);
            if (BVar2 != 0) {
                _local_8 = CONCAT22(local_6, 0x45);
                ppVar4 = process_struct_1010_20ba(&g_Struct372_1050_0ed0, _local_8);
                _local_8 = (ppVar4 & 0xffff0000 | param_1);
                pp_var1 = (*_local_8 + 0x10);
                (**pp_var1)(offset, ppVar4, 1);
            }
        }
    }
    return 1;
}

pub fn win_fn_1010_1656(param_1: *mut Struct370, param_2: u16, param_3: u16, param_4: u16) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut i_var3: i32;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let ppVar6: *mut pass1_struct_1;
    let mut u_var7: u32;
    let mut in_stack_0000000c: u16;
    let mut in_stack_0000ffe0: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_7ff6e820591: *mut Struct369;

    win_cleanup_1010_305a(param_1, param_2, param_3, param_3_00, in_stack_0000000c);
    if (param_1.field_0x16 == 3) {
        ppVar6 =
            process_struct_1010_20ba(_g_Struct372_1050_0ed0, CONCAT22(in_stack_0000ffe0, 0x32));
        u_var1 = param_1.field_0x32;
        u_var1 = (u_var1 + 0x42);
        u_var5 = (u_var1 >> 0x10);
        i_var4 = u_var1;
        u_var1 = (i_var4 + 0x16);
        u_var7 = pass1_1030_73a8((u_var1 + 4));
        i_var2 = u_var7;
        pass1_1010_7818(ppVar6, u_var7);
        u_var1 = (i_var4 + 0x16);
        i_var3 = i_var2;
        win_gui_fn_1010_79aa(ppVar6, 0, (u_var1 + 4));
        if (i_var3 == 0) {
            u_var1 = (i_var4 + 0x16);
            u_var1 = (u_var1 + 4);
            window_msg_func_1010_7300(ppVar6, 0, 0, i_var2, u_var1, (u_var1 >> 0x10));
        }
    }
    return;
}

pub fn free_rsrc_1010_4b3e(param_1: *mut Struct376) {
    let pu_var1: *mut u16;
    let pu_var2: *mut u32;
    let mut u_var3: i32;
    let ppcVar4: fn();
    let pu_var5: *mut u32;
    let mut u_var6: u32;
    let BVar7: bool;
    let mut i_var8: i32;
    let mut i_var9: i32;
    let mut u_var10: u16;
    let mut u_var11: u16;
    let mut unaff_cs: u16;
    let mut local_4: u16;

    u_var10 = (param_1 >> 0x10);
    i_var8 = param_1;
    param_1.ptr_a_lo = (s_SCForceMorale__s_for_colony__08l_1050_5024 + 6);
    (i_var8 + 2) = 0x1010;
    if ((i_var8 + 0x2a) != 0) {
        unaff_cs = SUB42(offset, 0);
        BVar7 = GlobalUnlock16((i_var8 + 0x2a));
        if (BVar7 == 0) {
            unaff_cs = SUB42(offset, 0);
            FreeResource16((i_var8 + 0x2a));
        }
    }
    (i_var8 + 0x2a) = 0;
    if ((i_var8 + 0x12) != 0) {
        local_4 = 0;
        while (true) {
            pu_var5 = (i_var8 + 0x12);
            pu_var1 = (pu_var5 + 8);
            unsafe {
                if (*pu_var1 == local_4 || *pu_var1 < local_4) {
                    break;
                }
                u_var11 = (*pu_var5 >> 0x10);
                i_var9 = *pu_var5;

                pu_var2 = (i_var9 + local_4 * 4);
                u_var3 = (i_var9 + local_4 * 4 + 2);
                if ((u_var3 | pu_var2) != 0) {
                    ppcVar4 = *pu_var2;
                    (**ppcVar4)(unaff_cs, pu_var2, u_var3, 1);
                }
            }
            local_4 = local_4 + 1;
        }
    }
    u_var6 = (i_var8 + 0x12);
    error_check_1000_17ce((u_var6 + 4));
    error_check_1000_17ce((i_var8 + 0x12));
    pu_var2 = (i_var8 + 0x16);
    u_var3 = (i_var8 + 0x18);
    if ((u_var3 | pu_var2) != 0) {
        unsafe {
            ppcVar4 = *pu_var2;
        }
        (**ppcVar4)(0x1000, pu_var2, u_var3, 1);
    }
    error_check_1000_17ce((i_var8 + 0x1a));
    pass1_1010_1d80(param_1);
    return;
}

pub fn load_rsrc_1010_4e9e(in_struct_1: *mut Struct60) -> SEGPTR {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let mut unlock_result: u16;
    let mut h_resource: u16;
    let handle: HGLOBAL16;
    let local_bx_5: *mut Struct60;
    let mut u_var3: u16;
    let SVar4: SEGPTR;

    u_var3 = (in_struct_1 >> 0x10);
    local_bx_5 = in_struct_1;
    if (local_bx_5.field_0x20 != 0) {
        if (local_bx_5.resource_handle != 0) {
            unlock_result = GlobalUnlock16(local_bx_5.resource_handle);
            if (unlock_result == 0) {
                FreeResource16(local_bx_5.resource_handle);
            }
        }
        u_var1 = local_bx_5.field_0x12;
        u_var2 = (u_var1 + 4);
        h_resource = FindResource16(
            0xa,
            *((u_var2 + local_bx_5.field_0x20 * 2) * 2 + 0x1384),
            g_h_instance_1050_038c,
        );
        handle = LoadResource16(h_resource, g_h_instance_1050_038c);
        local_bx_5.resource_handle = handle;
        if (handle != 0) {
            SVar4 = LockResource16(handle);
            return SVar4;
        }
    }
    return 0;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn send_msg_1040_c85a(param_1: u32) {
    _PTR_LOOP_1050_5efe = param_1;
    SendMessage16(0, 0xfa, 0x111, (param_1 + 0x1a));
    return;
}

pub fn window_msg_func_1008_97f2(
    param_1: *mut u32,
    param_2: *mut i32,
    param_3: i32,
    param_4_00: *mut u8,
    uparam_4: i32,
) {
    let pp_var1: fn();
    let BVar2: bool;
    let mut u_var3: u16;
    let mut i_var4: i32;
    let mut u_var5: i32;
    let unaff_cs: u8;
    let mut u_var6: u32;
    let u_var7: u8;
    let u_var8: u8;
    let mut cVar9: u8;

    i_var4 = param_1;
    u_var3 = (param_1 >> 0x10);
    u_var7 = SUB41(param_1, 0);
    if (param_4 == 0x2b) {
        if (unsafe { *param_2 == 4 }) {
            get_prop_1040_9566(param_2, param_3);
        } else {
            unsafe {
                pp_var1 = (*param_1 + 0x70);
            }
            (**pp_var1)();
        }
        u_var5 = 1;
        // goto LAB_1008_9a95;
    }
    u_var8 = (param_1 >> 0x10);
    if (param_4 < 0x2c) {
        unaff_cs = 8;
        match (param_4) {
            1 => {}
            2 => {
                unsafe {
                    pp_var1 = (*param_1 + 0x3c);
                }
                (**pp_var1)(&PTR_LOOP_1050_1008, i_var4, u_var3);
                SetWindowLong16(0, 0);
                BVar2 = IsWindow16((i_var4 + 0xbc));
                if (BVar2 != 0) {
                    PostMessage16(param_1, 199, 0x111, (i_var4 + 0xbc));
                }
            }
            3 => {
                unsafe {
                    pp_var1 = (*param_1 + 0x54);
                }
                (**pp_var1)(8, u_var7, u_var3, param_3, param_2);
            }
            // default:
            // goto switchD_1008_9b30_caseD_4;
            5 => {
                unsafe {
                    pp_var1 = (*param_1 + 0x58);
                }
                (**pp_var1)(8, u_var7, u_var8, param_3, param_2, param_4_00);
            }
            7 => {
                unsafe {
                    pp_var1 = (*param_1 + 0x50);
                }
                (**pp_var1)(8, i_var4, u_var3, param_4_00);
            }
            8 => {
                unsafe {
                    pp_var1 = (*param_1 + 0x74);
                }
                (**pp_var1)(8, i_var4, u_var3, param_4_00);
            }
            0xd => {
                unsafe {
                    pp_var1 = (*param_1 + 0x84);
                }
                i_var4 = (**pp_var1)(
                    8,
                    u_var7,
                    u_var8,
                    param_2,
                    CONCAT12(param_4_00._0_1_, param_3),
                );
                // goto LAB_1008_9ada;
            }
            0xf => {
                unsafe {
                    pp_var1 = (*param_1 + 0x34);
                }
                (**pp_var1)(&PTR_LOOP_1050_1008, i_var4, u_var3);
            }
            0x10 => {
                unsafe {
                    pp_var1 = (*param_1 + 0x38);
                }
                u_var6 = (**pp_var1)(&PTR_LOOP_1050_1008, i_var4, u_var3);
                return u_var6;
            }
            0x19 => {
                unsafe {
                    pp_var1 = (*param_1 + 0x78);
                }
                u_var3 = (**pp_var1)(
                    8,
                    u_var7,
                    u_var8,
                    param_2,
                    CONCAT12(param_4_00._0_1_, param_3),
                );
                return CONCAT22(0x1050, u_var3);
            }
            0x1c => {
                unsafe {
                    pp_var1 = (*param_1 + 0x30);
                }
                (**pp_var1)(8, i_var4, u_var3, param_4_00);
            }
        }
    } else {
        cVar9 = param_4;
        if (param_4 == 0x112) {
            if ((PTR_LOOP_1050_039a == 0x0)
                && (
                    unsafe { pp_var1 = (*param_1 + 0x48) },
                    unsafe { i_var4 = (**pp_var1)() },
                    i_var4 != 0,
                ))
            {
                def_wnd_proc_func_1008_9ce6(
                    param_1,
                    CONCAT13((param_3 >> 8), CONCAT12(param_3, param_2)),
                    CONCAT13(1, CONCAT12(cVar9, param_4_00)),
                );
            }
        } else {
            if (param_4 < 0x113) {
                if (param_4 == 0x86) {
                    unsafe {
                        pp_var1 = (*param_1 + 0x80);
                    }
                    u_var6 = (**pp_var1)();
                    return u_var6;
                }
                if (param_4 < 0x87) {
                    if (param_4 == 0x85) {
                        unsafe {
                            pp_var1 = (*param_1 + 0x7c);
                        }
                        u_var6 = (**pp_var1)();
                        return u_var6;
                    }
                    if (param_4 < 0x86) {
                        if (cVar9 == '7') {
                            return *(i_var4 + 0xc2);
                        }
                        if (cVar9 == 'A') {
                            unsafe {
                                pp_var1 = (*param_1 + 0x2c);
                            }
                            (**pp_var1)();
                            // goto switchD_1008_9b30_caseD_1;
                        }
                    }
                    // switchD_1008_9b30_caseD_4:
                    if ((param_4 < 0x400) || (0x7ffe < param_4)) {
                        u_var6 = def_wnd_proc_func_1008_9ce6(
                            param_1,
                            CONCAT22(param_3, param_2),
                            CONCAT22(param_4, param_4_00),
                        );
                        return u_var6;
                    }
                    unsafe {
                        pp_var1 = (*param_1 + 0x28);
                    }
                    (**pp_var1)(
                        unaff_cs,
                        u_var7,
                        u_var8,
                        param_2,
                        param_3,
                        CONCAT22(param_4, param_4_00),
                    );
                } else {
                    if (param_4 == 0x100) {
                        if (PTR_LOOP_1050_039a == 0x0) {
                            unsafe {
                                pp_var1 = (*param_1 + 0x6c);
                            }
                            (**pp_var1)();
                        }
                    } else {
                        if (param_4 == 0x102) {
                            if (PTR_LOOP_1050_039a == 0x0) {
                                unsafe {
                                    pp_var1 = (*param_1 + 0x68);
                                }
                                (**pp_var1)();
                            }
                        } else {
                            if (param_4 != 0x111) {}
                            // goto switchD_1008_9b30_caseD_4;
                            if ((param_4_00 != PTR_LOOP_1050_039c) && (PTR_LOOP_1050_039a == 0x0)) {
                                if (param_2 == 0x0) {
                                    unsafe {
                                        pp_var1 = (*param_1 + 0x40);
                                    }
                                    (**pp_var1)();
                                } else {
                                    unsafe {
                                        pp_var1 = (*param_1 + 0x44);
                                    }
                                    (**pp_var1)();
                                }
                            }
                        }
                    }
                }
            } else {
                if (param_4 == 0x204) {
                    if (PTR_LOOP_1050_039a == 0x0) {
                        unsafe {
                            pp_var1 = (*param_1 + 0x60);
                        }
                        (**pp_var1)();
                    }
                } else {
                    if (param_4 < 0x205) {
                        if (param_4 == 0x113) {
                            if (_PTR_LOOP_1050_0388 != 0) {
                                pass1_1008_932a(_PTR_LOOP_1050_0388);
                            }
                        } else {
                            if (param_4 == 0x117) {
                                if (param_3 == 0) {
                                    unsafe {
                                        pp_var1 = (*param_1 + 0x4c);
                                    }
                                    (**pp_var1)();
                                } else {
                                    unsafe {
                                        pp_var1 = (*param_1 + 0x20);
                                    }
                                    (**pp_var1)();
                                }
                            } else {
                                if (param_4 != 0x201) {}
                                // goto switchD_1008_9b30_caseD_4;
                                if (PTR_LOOP_1050_039a == 0x0) {
                                    unsafe {
                                        pp_var1 = (*param_1 + 0x5c);
                                    }
                                    (**pp_var1)();
                                }
                            }
                        }
                    } else {
                        if (param_4 == 0x210) {
                            unsafe {
                                pp_var1 = (*param_1 + 100);
                            }
                            (**pp_var1)();
                        } else {
                            if (param_4 == 0x30f) {
                                // LAB_1008_9af8:
                                unsafe {
                                    pp_var1 = (*param_1 + 0x8c);
                                }
                                i_var4 = (**pp_var1)();
                                // LAB_1008_9ada:
                                return i_var4;
                            }
                            if (param_4 == 0x311) {
                                unsafe {
                                    pp_var1 = (*param_1 + 0x88);
                                }
                                i_var4 = (**pp_var1)();
                                if (i_var4 != 0) {}
                                // goto LAB_1008_9af8;
                            } else {
                                if (param_4 != 0x3b9) {}
                                // goto switchD_1008_9b30_caseD_4;
                                unsafe {
                                    pp_var1 = (*param_1 + 0x24);
                                }
                                (**pp_var1)();
                            }
                        }
                    }
                }
            }
        }
    }
    // switchD_1008_9b30_caseD_1:
    u_var5 = 0;
    // LAB_1008_9a95:
    return u_var5;
}

pub fn pass1_1008_9c16(param_1: u16, param_2: u32, param_3: u32) {
    def_wnd_proc_func_1008_9ce6(
        CONCAT22(param_2, param_1),
        CONCAT22(param_3, (param_2 >> 0x10)),
        CONCAT22(0x85, (param_3 >> 0x10)),
    );
    return;
}

pub fn def_wn_proc_1008_9c30(param_1: u16, param_2: u32, param_3: u32) {
    def_wnd_proc_func_1008_9ce6(
        CONCAT22(param_2, param_1),
        CONCAT22(param_3, (param_2 >> 0x10)),
        CONCAT22(0x86, (param_3 >> 0x10)),
    );
    return;
}

pub fn def_wnd_proc_func_1008_9ce6(param_1: &mut Vec<u8>, param_2: u32, param_3: u32) -> LRESULT {
    let LVar1: LRESULT;

    LVar1 = DefWindowProc16(param_2, param_3, (param_3 >> 0x10), (param_1 + 8));
    return LVar1;
}

pub fn post_win_msg_1008_a0e4(
    param_1: *mut pass1_struct_1,
    param_2: u32,
    param_3: u16,
    param_4: u16,
    param_5: u32,
    param_6: u16,
) -> u8 {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut bVar3: bool;
    let u_var4: u8;
    let mut u_var5: i32;
    let mut u_var7: u32;
    let puVar8: *mut u16;
    let mut extraout_dx: i32;
    let mut extraout_dx_00: i32;
    let mut i_var9: i32;
    let mut u_var10: u16;
    let mut unaff_ss: u16;
    let mut local_20: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: [u8; 8];
    let BVar6: bool;

    u_var10 = (param_1 >> 0x10);
    i_var9 = param_1;
    pass1_1008_5784(CONCAT22(unaff_ss, local_a), (i_var9 + 10));
    bVar3 = false;
    while {
        u_var7 = ZEXT24(local_a);
        pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
        u_var5 = u_var7;
        if ((extraout_dx | u_var5) == 0) {}
        // goto LAB_1008_a146;
        (u_var5 + 4) != param_6
    } {}
    (u_var5 + 0xc) = (u_var5 + 0xc) + param_3;
    u_var7 = (u_var5 + 0xe) + param_2;
    (u_var5 + 0xe) = u_var7;
    bVar3 = true;
    // LAB_1008_a146:
    u_var4 = u_var7;
    if (!bVar3) {
        puVar8 = _PTR_LOOP_1050_03a0;
        alloc_mem_1000_07fc(_PTR_LOOP_1050_03a0);
        u_var5 = puVar8;
        local_e = puVar8 & 0xffff | extraout_dx_00 << 0x10;
        if ((extraout_dx_00 | u_var5) == 0) {
            local_e = 0;
        } else {
            local_e = s_1_1050_389a;
            (u_var5 + 2) = &PTR_LOOP_1050_1008;
            (u_var5 + 4) = param_6;
            (u_var5 + 6) = param_5;
            (u_var5 + 10) = param_4;
            (u_var5 + 0xc) = param_3;
            (u_var5 + 0xe) = param_2;
            local_e = 0xad8e;
            (u_var5 + 2) = &PTR_LOOP_1050_1008;
            puVar8 = local_e;
        }
        u_var4 = puVar8;
        u_var1 = (i_var9 + 10);
        ppc_var2 = ((i_var9 + 10) + 8);
        ppc_var2(0x1000, u_var1, (u_var1 >> 0x10), local_e, (local_e >> 0x10));
    }
    if (param_6 == 0x14) {
        BVar6 = PostMessage16(0, 0xfc, 0x111, g_h_window);
        u_var4 = BVar6;
    }
    return u_var4;
}

pub fn create_win_1008_9760(in_win_struct: *mut win_struct_42) -> *mut Struct199 {
    let mut win_handle_1: u16;
    let mut in_dx: u16;
    let win_struct: *mut win_struct_42;
    let mut u_var1: i32;

    u_var1 = (in_win_struct >> 0x10);
    win_struct = in_win_struct;
    if (win_struct.win_handle_0x8 == 0) {
        win_handle_1 = CreateWindowEx16(
            (in_win_struct & 0xffff | u_var1 << 0x10),
            g_h_instance_1050_038c,
            win_struct.menu_handle_0xca,
            win_struct.parent_window_0xbc,
            win_struct.height_0xba,
            win_struct.width_0xb8,
            win_struct.y_0xb6,
            win_struct.x_0xb4,
            win_struct.style_0xac,
            &g_string_1050_039e,
            (in_win_struct & 0xffff0000 | ZEXT24(&win_struct.class_name_0x5b)),
            win_struct.style_0xb0,
        );
        win_struct.win_handle_0x8 = win_handle_1;
    }
    if (win_struct.win_handle_0x8 == 0) {
        win_handle_1 = call_fn_ptr_1000_24cd(0);
    }
    return CONCAT22(in_dx, win_handle_1);
}

pub fn print_fn_1008_97c8(param_1: u32) {
    let mut u_var1: u16;
    let unaff_ss: HWND16;
    let local_22: PAINTSTRUCT16;

    u_var1 = (param_1 >> 0x10);
    BeginPaint(CONCAT22(unaff_ss, &local_22), (param_1 + 8));
    local_22.hdc = (param_1 + 8);
    EndPaint(&local_22, unaff_ss);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn process_win_msg_1008_9498() -> WPARAM16 {
    let mut b_result: u16;
    let b_var1: bool;
    let i_var2: u16;
    let mut h_wnd_dlg: u16;
    let mut local_18: u16;
    let mut message: [u8; 18];
    let mut local_4: u16;

    // LAB_1008_949c:
    b_result = GetMessage16(0, 0, 0, message);
    if (b_result == 0) {
        return message._4_2_;
    }
    if ((_PTR_LOOP_1050_5bc8 + 8) != 0) {}
    // goto code_r0x100894cd;
    // goto LAB_1008_94dc;
    // code_r0x100894cd:
    b_var1 = IsDialogMessage16(message, h_wnd_dlg);
    if (b_var1 == 0) {
        // LAB_1008_94dc:
        if (g_haccel_1050_0398 != 0x0) {
            i_var2 = TranslateAccelerator16(
                CONCAT22(h_wnd_dlg, message),
                g_haccel_1050_0398,
                g_h_window,
            );
            if (i_var2 != 0) {}
            // goto LAB_1008_949c;
        }
        TranslateMessage16(message);
        DispatchMessage16(message);
    }
    // goto LAB_1008_949c;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn process_win_msg_1008_9510(param_1: *mut i32) {
    let b_var1: bool;
    let i_var2: u16;
    let unaff_ss: HWND16;
    let mut local_16: u16;
    let mut msg: [u8; 18];

    // LAB_1008_9578:
    if (unsafe { *param_1 != 0 }) {
        b_var1 = GetMessage16(0, 0, 0, msg);
        if (b_var1 != 0) {
            if ((_PTR_LOOP_1050_5bc8 + 8) != 0) {}
            // goto code_r0x10089538;
            // goto LAB_1008_9547;
        }
    }
    return;
    // code_r0x10089538:
    b_var1 = IsDialogMessage16(msg, unaff_ss);
    if (b_var1 == 0) {
        // LAB_1008_9547:
        if (g_haccel_1050_0398 != 0x0) {
            i_var2 =
                TranslateAccelerator16(CONCAT22(unaff_ss, msg), g_haccel_1050_0398, g_h_window);
            if (i_var2 != 0) {}
            // goto LAB_1008_9578;
        }
        TranslateMessage16(msg);
        DispatchMessage16(msg);
    }
    // goto LAB_1008_9578;
}

pub fn send_win_msg_1008_9640(param_1: u32, param_2: WPARAM16) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    if ((param_1 + 8) != 0) {
        SendMessage16(0, param_2, 0x86, (param_1 + 8));
    }
    return;
}

pub fn send_win_msg_1008_84ba(param_1: u16, param_2: *mut HWND16) {
    let mut i_var1: i32;
    let mut u_var2: u16;
    let mut local_4: u16;

    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    if ((*(i_var1 + 4) & 4) == 0) {
        if ((*(i_var1 + 4) & 8) == 0) {
            return;
        }
        local_4 = 1;
    } else {
        local_4 = 0;
    }
    SendMessage16(*(i_var1 + 2), local_4, 0x115, *param_1);
    return;
}

// WARNING: Variable defined which should be unmapped: local_1e

pub fn reg_class_1008_818c(param_1: u32) {
    let b_var1: bool;
    let AVar2: ATOM;
    let mut unaff_ss: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    local_6 = param_1 + 4;
    b_var1 = GetClassInfo16(&local_1c, CONCAT22(local_6, unaff_ss), param_1._2_2_);
    if (b_var1 == 0) {
        local_1c = (param_1 + 0x54);
        local_1a = 0x84f2;
        local_18 = &PTR_LOOP_1050_1008;
        local_16 = 0x40000;
        local_12 = g_h_instance_1050_038c;
        local_10 = 0;
        local_e = (param_1 + 0x58);
        local_c = (param_1 + 0x56);
        local_a = 0;
        local_4 = param_1._2_2_;
        AVar2 = RegisterClass16(&local_1c);
        if (AVar2 == 0) {
            call_fn_ptr_1000_24cd(0);
        }
    }
    return;
}

pub fn free_proc_and_check_err_1008_3cd6(param_1: u32, param_2: u8) {
    free_proc_inst_1040_911e(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pos_msg_1008_3d20(param_1: u32) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    PostMessage16(0, (param_1 + 0xcc), 0x111, (param_1 + 0xbc));
    return;
}

pub fn process_win_msg_1008_54aa(
    in_Struct97_1: u16,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    in_h_instance: u16,
) {
    let pp_var1: fn();
    let local_AX__1: *mut u8;
    let mut local_DX_57: u16;
    let mut local_DX_81: u16;
    let mut local_DX_105: u16;
    let mut local_DX_129: u16;
    let mut local_DX_153: u16;
    let extraout_dx: *mut Struct199;
    let struct_a: *mut Struct199;
    let mut local_DX_249: u16;
    let mut u_var2: u16;
    let mut local_1e: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let func_ptr_c: &mut Vec<u8>;
    let mut temp_5f75f7f23c: u32;

    if (param_3_00 != 0) {
        return;
    }
    dos3_call_1000_435c(0);
    set_global_uint_1000_4d0c();
    check_and_set_global_1000_1fea();
    init_struct_1000_1902(0x0, 0x32, 0, 0x12);
    _PTR_LOOP_1050_03a0 = CONCAT22(local_DX_57, local_AX__1);
    init_struct_1000_1902(0x0, 100, 0, 0xc);
    _PTR_LOOP_1050_029c = CONCAT22(local_DX_81, local_AX__1);
    init_struct_1000_1902(0x0, 100, 0, 0x10);
    _PTR_LOOP_1050_4fb8 = CONCAT22(local_DX_105, local_AX__1);
    init_struct_1000_1902(0x0, 100, 0, 0xe);
    _PTR_LOOP_1050_68a2 = CONCAT22(local_DX_129, local_AX__1);
    init_struct_1000_1902(0x0, 500, 0, 0x42);
    _PTR_LOOP_1050_5744 = CONCAT22(local_DX_153, local_AX__1);
    init_struct_1000_1902(0x0, 0x32, 0, 6);
    g_h_instance_1050_038c = in_h_instance;
    PTR_LOOP_1050_038e = param_3_00;
    PTR_LOOP_1050_0390 = in_Struct97_1;
    struct_a = extraout_dx;
    PTR_LOOP_1050_5768 = local_AX__1;
    PTR_LOOP_1050_576a = extraout_dx;
    pass1_fn_1008_60e8();
    _PTR_LOOP_1050_0392 = CONCAT22(struct_a, in_Struct97_1);
    process_struct_1000_179c(0xc, struct_a);
    if ((struct_a | in_Struct97_1) == 0) {
        in_Struct97_1 = 0;
        u_var2 = 0;
    } else {
        set_struct_1008_0000(in_Struct97_1);
        u_var2 = local_DX_249;
    }
    func_ptr_c = CONCAT22(u_var2, in_Struct97_1);
    if (_PTR_LOOP_1050_0392 != 0) {
        // WARNING: Load size is inaccurate
        unsafe {
            pp_var1 = (*func_ptr_c + 4);
        }
        (**pp_var1)(0x1000, in_Struct97_1, u_var2, _PTR_LOOP_1050_0392);
    }
    local_1e = CONCAT22(u_var2, in_Struct97_1);
    // WARNING: Load size is inaccurate
    unsafe {
        temp_5f75f7f23c = *func_ptr_c;
    }
    pp_var1 = temp_5f75f7f23c + 4;
    (**pp_var1)();
    process_win_msg_1008_9498(local_1e);
    if (func_ptr_c != 0) {
        pp_var1 = temp_5f75f7f23c;
        (**pp_var1)(0x1000, in_Struct97_1, u_var2, 1);
    }
    free_mem_1000_1b68(_PTR_LOOP_1050_03a0, (_PTR_LOOP_1050_03a0 >> 0x10));
    free_mem_1000_1b68(_PTR_LOOP_1050_029c, (_PTR_LOOP_1050_029c >> 0x10));
    free_mem_1000_1b68(_PTR_LOOP_1050_4fb8, (_PTR_LOOP_1050_4fb8 >> 0x10));
    free_mem_1000_1b68(_PTR_LOOP_1050_68a2, (_PTR_LOOP_1050_68a2 >> 0x10));
    free_mem_1000_1b68(_PTR_LOOP_1050_5744, (_PTR_LOOP_1050_5744 >> 0x10));
    return;
}

pub fn reg_class_1040_98c0(param_1: u32) {
    let b_var1: bool;
    let AVar2: ATOM;
    let mut unaff_ss: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    local_6 = param_1 + 4;
    b_var1 = GetClassInfo16(&local_1c, CONCAT22(local_6, unaff_ss), param_1._2_2_);
    if (b_var1 == 0) {
        local_1c = (param_1 + 0x54);
        local_1a = 0x9cde;
        local_18 = &PTR_LOOP_1050_1040;
        local_16 = 0x40000;
        local_12 = g_h_instance_1050_038c;
        local_10 = 0;
        local_e = (param_1 + 0x58);
        local_c = (param_1 + 0x56);
        local_a = 0;
        local_4 = param_1._2_2_;
        AVar2 = RegisterClass16(&local_1c);
        if (AVar2 == 0) {
            call_fn_ptr_1000_24cd(0);
        }
    }
    return;
}

pub fn get_prop_1040_9566(param_1: *mut i32) {
    let mut u_var1: u16;
    let mut i_var2: i32;
    let ppc_var3: fn();
    let h_var4: HANDLE16;
    let HVar5: HANDLE16;
    let mut i_var6: i32;
    let mut u_var7: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var7 = (param_1 >> 0x10);
    i_var6 = param_1;
    if (unsafe { *param_1 == 4 }) {
        u_var1 = (i_var6 + 0xc);
        h_var4 = GetProp16(s_thisHi_1050_5e6f, (i_var6 + 10));
        HVar5 = GetProp16(s_thisLo_1050_5e68, (i_var6 + 10));
        if ((h_var4 | HVar5) != 0) {
            i_var2 = (i_var6 + 6);
            if (i_var2 == 1) {
                _local_c = CONCAT22(h_var4, u_var1);
                ppc_var3 = (*_local_c + 0xc);
                (**ppc_var3)(offset, u_var1, h_var4, (i_var6 + 8));
                return;
            }
            if (i_var2 == 2) {
                _local_c = CONCAT22(h_var4, u_var1);
                ppc_var3 = (*_local_c + 0x10);
                (**ppc_var3)(offset, u_var1, h_var4, (i_var6 + 8));
                return;
            }
            if (i_var2 == 4) {
                _local_c = CONCAT22(h_var4, u_var1);
                ppc_var3 = (*_local_c + 0x18);
                (**ppc_var3)(offset, u_var1, h_var4, *(i_var6 + 8) & 0x10);
                return;
            }
        }
    }
    return;
}

pub fn send_win_msg_1040_93e6(param_1: u32) -> LRESULT {
    let mut u_var1: u16;
    let LVar2: LRESULT;

    u_var1 = (param_1 >> 0x10);
    LVar2 = SendMessage16(0, (param_1 + 0x1c), 0x111, (param_1 + 0x1a));
    return LVar2;
}

pub fn send_msg_1040_9404(param_1: u32) -> LRESULT {
    let mut u_var1: u16;
    let LVar2: LRESULT;
    u_var1 = (param_1 >> 0x10);
    LVar2 = SendMessage16(0, (param_1 + 0x1c), 0x111, (param_1 + 0x1a));
    return LVar2;
}

pub fn make_proc_inst_1040_8fb8(
    in_Struct1: *mut Struct74,
    param_2: u16,
    param_3: u32,
    param_4: u16,
    param_5: u16,
    param_6: u16,
    param_7: u16,
    param_8: u16,
) {
    let mut u_var1: u16;
    let mut u_var2: u16;
    let mut in_dx: u16;
    let mut extraout_dx: u16;
    let mut extraout_dx_00: u16;
    let mut extraout_dx_01: u16;
    let local_bx_4: *mut Struct74;
    let mut hinstance: u16;
    let mut in_stack_00000006: u16;

    *_in_Struct1 = s_1_1050_389a;
    in_Struct1.field_0x2 = &PTR_LOOP_1050_1008;
    &in_Struct1.field_0x4 = 0;
    &in_Struct1.field_0x8 = 0;
    &in_Struct1.field_0xc = 0;
    &in_Struct1.field_0x10 = 0;
    in_Struct1.field_0x14 = 0;
    in_Struct1.field_0x18 = 0;
    in_Struct1.field_0x1a = param_8;
    in_Struct1.field_0x1c = param_7;
    in_Struct1.field_0x36 = 5;
    u_var1 = 0;
    in_Struct1.field_0x38 = 0;
    in_Struct1.field_0x3a = 0;
    in_Struct1.field_0x3c = 2;
    in_Struct1.field_0x3e = 0;
    in_Struct1.field_0x40 = param_2;
    *_in_Struct1 = 0x9800;
    in_Struct1.field_0x2 = &PTR_LOOP_1050_1040;
    u_var2 = in_Struct1.field_0x36;
    in_Struct1.field_0x28 = u_var2;
    in_Struct1.field_0x26 = u_var2;
    in_Struct1.field_0x2c = 0;
    in_Struct1.field_0x2a = 0;
    if ((param_6 != 0) && (param_5 != 0)) {
        in_Struct1.field_0x38 = 1;
        mixed_fn_1010_830a(_g_struct_73_1050_14cc, param_6);
        in_Struct1.field_0x8 = u_var1;
        in_Struct1.field_0xa = extraout_dx;
        hinstance = 0x1010;
        mixed_fn_1010_830a(_g_struct_73_1050_14cc, param_5);
        in_Struct1.field_0xc = u_var1;
        in_Struct1.field_0xe = extraout_dx_00;
        if (param_4 == 0) {
            &in_Struct1.field_0x10 = 0;
            in_dx = extraout_dx_00;
        } else {
            hinstance = 0x1010;
            mixed_fn_1010_830a(_g_struct_73_1050_14cc, param_4);
            in_Struct1.field_0x10 = u_var1;
            in_Struct1.field_0x12 = extraout_dx_01;
            in_dx = extraout_dx_01;
        }
    }
    u_var2 = in_Struct1.field_0x36;
    in_Struct1.field_0x30 = u_var2;
    in_Struct1.field_0x2e = u_var2;
    in_Struct1.field_0x32 = 0;
    if (param_3 != 0) {
        hinstance = &PTR_LOOP_1050_1008;
        pass1_fn_1008_60e8(param_3);
        in_Struct1.field_0x4 = u_var2;
        in_Struct1.field_0x6 = in_dx;
    }
    in_Struct1.field_0x22 = 0;
    in_Struct1.field_0x1e = 0;
    in_Struct1.field_0x20 = 0;
    if (_g_proc_inst_1050_5e18 == 0) {
        _g_proc_inst_1050_5e18 =
            MakeProcInstance16(hinstance, CONCAT22(0x9684, g_h_instance_1050_038c));
    }
    PTR_LOOP_1050_5e16 = PTR_LOOP_1050_5e16 + 1;
    return;
}

pub fn pass1_1040_8f98(param_1: u32) {
    let pbVar1: *mut u8;
    let mut b_var2: u8;
    let mut cVar3: u8;
    let mut u_var4: i32;
    let mut in_AX: u16;
    let mut u_var5: u16;
    let mut in_CX: u16;
    let mut bVar6: u8;
    let mut in_dx: i32;
    let mut u_var7: i32;
    let mut extraout_dx: u16;
    let mut extraout_dx_00: i32;
    let mut extraout_dx_01: i32;
    let mut in_BX: i32;
    let mut bVar10: u8;
    let mut i_var9: i32;
    let local_bx_272: *mut Struct359;
    let pu_var11: *mut u16;
    let unaff_BP: *mut u16;
    let unaff_si: *mut u8;
    let mut u_var12: u16;
    let unaff_cs: HANDLE16;
    let mut unaff_ss: u16;
    let mut in_CF: u8;
    let mut bVar13: bool;
    let mut bVar14: bool;
    let mut uStack0008: u16;
    let mut uStack000a: u32;
    let mut uStack000e: u32;
    let mut uStack0012: u32;
    let mut uStack0016: u16;
    let mut uStack0024: u16;
    let mut uStack0026: u16;
    let mut uStack0028: u16;
    let mut uStack002a: u16;
    let mut uStack0034: u16;
    let mut uStack0036: u16;
    let mut uStack0038: u16;
    let mut uStack003a: u16;
    let mut uStack003c: u16;
    let pu_var15: *mut u16;
    let lVar16: u32;
    let mut in_stack_0000bf2a: u16;
    let mut in_stack_0000bf2c: u16;
    let local_4e: u8;
    let puStack34: *mut u8;
    let mut bVar8: u8;

    puStack34 = &stack0xfffe;
    pu_var11 = &stack0xfffe;
    cVar3 = '\x0f';
    while {
        unaff_BP = unaff_BP + -1;
        pu_var11 = pu_var11 + -1;
        unsafe {
            *pu_var11 = *unaff_BP;
        }
        cVar3 = cVar3 + -1;
        '\0' < cVar3
    } {}
    bVar10 = (in_BX >> 8);
    unaff_si[in_BX] = bVar10;
    bVar8 = (in_dx + 1 >> 8);
    b_var2 = bVar8 + bVar10;
    bVar13 = CARRY1(bVar8, bVar10) || CARRY1(b_var2, in_CF);
    u_var4 = in_dx + 1 & 0xff;
    u_var7 = u_var4 | (b_var2 + in_CF) << 8;
    lVar16 = CONCAT22(in_CX, u_var7);
    pu_var15 = CONCAT22(&stack0xbf2a, &stack0xfffe);
    pbVar1 = unaff_si + in_BX;
    bVar6 = u_var4;
    unsafe {
        b_var2 = *pbVar1;
        bVar8 = *pbVar1 - bVar6;
        bVar14 = *pbVar1 < bVar6 || bVar8 < bVar13;
        *pbVar1 = bVar8 - bVar13;
        let pb_var1_val = unsafe { *pbVar1 };
        if (pb_var1_val != 0
            && (SBORROW1(b_var2, bVar6) != SBORROW1(bVar8, bVar13)) == (pb_var1_val < '\0'))
        {
            pbVar1 = unaff_si;
            bVar13 = CARRY1(*pbVar1, bVar10) || CARRY1(*pbVar1 + bVar10, bVar14);
            *pbVar1 = *pbVar1 + bVar10 + bVar14;
            b_var2 = local_4e + in_BX;
            bVar14 = CARRY1(local_4e, in_BX) || CARRY1(b_var2, bVar13);
            local_4e = b_var2 + bVar13;
            pbVar1 = unaff_si + -0x4f;
            b_var2 = *pbVar1;
            bVar8 = *pbVar1;
            *pbVar1 = bVar8 + bVar10 + bVar14;
            pbVar1 = unaff_si + -0x78;
            *pbVar1 = *pbVar1 + in_CX + (CARRY1(b_var2, bVar10) || CARRY1(bVar8 + bVar10, bVar14));
            *pu_var15 = s_1_1050_389a;
            uStack0008 = 0;
            uStack000a = 0;
            uStack000e = 0;
            uStack0012 = 0;
            uStack0016 = 0;
            uStack0034 = 5;
            u_var5 = 0;
            uStack0036 = 0;
            uStack0038 = 0;
            uStack003a = 2;
            uStack003c = 0;
            *pu_var15 = 0x9800;
            uStack0026 = 5;
            uStack0024 = 5;
            uStack002a = 0;
            uStack0028 = 0;
            puStack34 = &stack0xfffe;
            if ((in_stack_0000bf2c != 0) && (puStack34 = &stack0xfffe, in_stack_0000bf2a != 0)) {
                uStack0036 = 1;
                puStack34 = &stack0xfffe;
                mixed_fn_1010_830a(_g_struct_73_1050_14cc, in_stack_0000bf2c);
                u_var12 = (pu_var15 >> 0x10);
                (pu_var15 + 8) = u_var5;
                (pu_var15 + 10) = extraout_dx;
                unaff_cs = 0x1010;
                mixed_fn_1010_830a(_g_struct_73_1050_14cc, in_stack_0000bf2a);
                u_var12 = (pu_var15 >> 0x10);
                i_var9 = pu_var15;
                (i_var9 + 0xc) = u_var5;
                (i_var9 + 0xe) = extraout_dx_00;
                if (in_AX == 0) {
                    (i_var9 + 0x10) = 0;
                    u_var7 = extraout_dx_00;
                } else {
                    unaff_cs = 0x1010;
                    mixed_fn_1010_830a(_g_struct_73_1050_14cc, in_AX);
                    u_var12 = (pu_var15 >> 0x10);
                    (pu_var15 + 0x10) = u_var5;
                    (pu_var15 + 0x12) = extraout_dx_01;
                    u_var7 = extraout_dx_01;
                }
            }
            u_var12 = (pu_var15 >> 0x10);
            local_bx_272 = pu_var15;
            u_var5 = local_bx_272.field_0x36;
            local_bx_272.field_0x30 = u_var5;
            local_bx_272.field_0x2e = u_var5;
            local_bx_272.field_0x32 = 0;
            if (lVar16 != 0) {
                unaff_cs = &PTR_LOOP_1050_1008;
                pass1_fn_1008_60e8();
                u_var12 = (pu_var15 >> 0x10);
                (pu_var15 + 4) = u_var5;
                (pu_var15 + 6) = u_var7;
            }
            u_var5 = (pu_var15 >> 0x10);
            i_var9 = pu_var15;
            (i_var9 + 0x22) = 0;
            (i_var9 + 0x1e) = 0;
            (i_var9 + 0x20) = 0;
            if (_g_proc_inst_1050_5e18 == 0) {
                _g_proc_inst_1050_5e18 =
                    MakeProcInstance16(unaff_cs, CONCAT22(0x9684, g_h_instance_1050_038c));
            }
            PTR_LOOP_1050_5e16 = PTR_LOOP_1050_5e16 + 1;
            return;
        }
        if (*pbVar1 != 0) {
            error_check_1000_17ce(param_1);
        }
    }
    return;
}

pub fn process_struct_1040_8478(
    param_1: u32,
    param_2: u16,
    param_3: u32,
    param_4: u32,
    param_5: u16,
) -> *mut u16 {
    let extraout_var: u32;
    let mut u_var1: u32;
    let mut in_dx: u16;
    let local_bx_27: *mut Struct361;
    let mut u_var2: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    process_struct_1040_7728(param_1, (&PTR_LOOP_1050_0000 + 1), 0, 0xfc3, param_5);
    u_var2 = (param_1 >> 0x10);
    local_bx_27 = param_1;
    local_bx_27.field_0x8e = 0;
    u_var1 = (extraout_var & 0xffff00) << 8;
    local_bx_27.field_0x98 = param_2;
    local_bx_27.field_0x9a = 0;
    local_bx_27.field_0xb2 = 0;
    param_1 = 0x8ddc;
    local_bx_27.field_0x2 = &PTR_LOOP_1050_1040;
    local_bx_27.field_0x9e = 0;
    local_bx_27.field_0xa2 = 300;
    pass1_fn_1008_60e8(param_4);
    local_bx_27.field_0x90 = u_var1;
    local_bx_27.field_0x92 = in_dx;
    pass1_fn_1008_60e8(param_3);
    local_bx_27.field_0x94 = u_var1;
    local_bx_27.field_0x96 = in_dx;
    win_fn_1040_8b92(param_1);
    PTR_LOOP_1050_5df8 = 0x0;
    return (u_var1 & 0xffff0000 | param_1 & 0xffff);
}

pub fn post_win_msg_1040_7f1c(param_1: u32, param_2: i32) -> u16 {
    let mut i_var1: i32;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    if ((i_var1 + 0x78) != 0) {
        return 0;
    }
    if ((i_var1 + 0x60) != param_2) {
        (i_var1 + 0x60) = param_2;
        PostMessage16(0, 0, 0x85, (i_var1 + 6));
    }
    return 1;
}

pub fn post_win_msg_1040_7f56(param_1: u32, param_2: *mut char) {
    copy_string_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 0x10)), param_2);
    PostMessage16(0, 0, 0x85, (param_1 + 6));
    return;
}

pub fn post_win_msg_1040_7b3c(param_1: *mut u32, param_2: u16, param_3: u16, param_4: i32) -> u16 {
    let pp_var1: fn();

    if ((param_2 == 1) || (param_2 == 2)) {
        unsafe {
            pp_var1 = (*param_1 + 0x14);
        }
        (**pp_var1)();
    } else {
        if (param_2 == 0x6f) {
            unsafe {
                pp_var1 = (*param_1 + 0x2c);
            }
            (**pp_var1)();
        } else {
            if (param_2 != 0x12e) {
                return 0;
            }
            PostMessage16(0, 0xf060, 0x112, (param_1 + 6));
        }
    }
    return 1;
}

pub fn post_win_msg_1040_672e(param_1: i32, param_2: u16, param_3: u16, param_4: u32) {
    let mut u_var1: u16;

    if (param_3._2_2_ == (s_vrpal_bmp_1050_183a + 7)) {
        msg_box_1040_64ca(param_1, param_2);
    } else {
        if (param_3._2_2_ == (s_logo_bmp_1050_1850 + 1)) {
            u_var1 = 0x2a;
        } else {
            if (param_3._2_2_ != (s_logo_bmp_1050_1850 + 2)) {
                post_win_msg_1040_7b3c(param_1, param_2, param_3_00, param_3, param_3._2_2_);
                return;
            }
            u_var1 = 0x29;
        }
        pass1_1038_af40(_g_Struct112_a, *(param_1 + 8), u_var1);
        PostMessage16(0, 2, 0x111, (param_1 + 6));
    }
    return;
}

pub fn send_win_msg_1040_4cf4(param_1: u32) {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let mut u_var3: u32;
    let h_wnd: HWND16;
    let mut i_var4: i32;
    let mut i_var5: i32;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let LVar8: LRESULT;
    let mut local_58: u16;
    let mut local_56: u16;
    let mut local_54: u16;
    let mut local_52: [u8; 80];

    u_var6 = (param_1 >> 0x10);
    i_var5 = param_1;
    h_wnd = GetDlgItem16(6000, (i_var5 + 6));
    LVar8 = SendMessage16(0, 0, 0x407, h_wnd);
    if (LVar8 != 0xffff) {
        SendMessage16(CONCAT22(unaff_ss, local_52), LVar8, 0x408, h_wnd);
    }
    u_var3 = (i_var5 + 0x90);
    u_var1 = (i_var5 + 0x94);
    i_var4 = pass1_1010_ae12(
        u_var1,
        (u_var1 >> 0x10),
        CONCAT22(unaff_ss, local_52),
        (u_var3 + 10),
    );
    if (i_var4 != -1) {
        u_var1 = (i_var5 + 0x90);
        u_var7 = (u_var1 >> 0x10);
        i_var4 = u_var1;
        u_var2 = (i_var4 + 6);
        local_58 = u_var2;
        pass1_1010_ae92((i_var5 + 0x94), local_58, (i_var4 + 10), u_var2);
    }
    return;
}

pub fn win_fn_1040_3374(param_1: u32, param_2: *mut u32, param_3: HWND16) {
    let pp_var1: fn();
    let mut u_var2: u16;
    let mut u_var3: u32;
    let mut extraout_dx: u16;
    let mut extraout_dx_00: u16;
    let mut extraout_dx_01: u16;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let LVar6: LRESULT;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var5 = SUB42(offset, 0);
    LVar6 = SendMessage16(0, 0, 0x40b, param_3);
    u_var2 = LVar6;
    u_var4 = (param_2 >> 0x10);
    unsafe {
        pp_var1 = (*param_2 + 0x10);
    }
    (**pp_var1)(offset, param_2, u_var4);
    _local_6 = CONCAT22(extraout_dx, u_var2);
    local_a = 0;
    loop {
        if (_local_6 <= local_a) {
            return;
        }
        unsafe {
            pp_var1 = (*param_2 + 4);
        }
        u_var3 = _local_6;
        (**pp_var1)(u_var5, param_2, u_var4, local_a, (local_a >> 0x10));
        u_var2 = u_var3;
        pass1_1018_3a7a(
            *(param_1 + 0x96),
            CONCAT13((extraout_dx_00 >> 8), CONCAT12(extraout_dx_00, u_var2)),
        );
        LVar6 = SendMessage16(CONCAT22(extraout_dx_01, u_var2), 0, 0x403, param_3);
        u_var5 = 0x1000;
        error_check_1000_17ce(CONCAT22(extraout_dx_01, u_var2));
        if (LVar6 == -1) {
            break;
        }
        if (LVar6 == -2) {
            return;
        }
        local_a = local_a + 1;
    }
    return;
}

pub fn send_msg_1040_323c(param_1: u32) -> LRESULT {
    let mut i_var1: i32;
    let mut u_var2: u16;
    let LVar3: LRESULT;
    let mut local_a: u16;
    let mut local_6: u16;
    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    LVar3 = SendMessage16(0, 0, 0x407, (i_var1 + 0x92));
    SendMessage16(0, 0, 0x407, (i_var1 + 0x94));
    SendMessage16(
        param_1 & 0xffff0000 | (i_var1 + 0x9a),
        LVar3,
        0x408,
        (i_var1 + 0x92),
    );
    LVar3 = SendMessage16(
        param_1 & 0xffff0000 | (i_var1 + 0x19a),
        0x408,
        0x408,
        (i_var1 + 0x94),
    );
    return LVar3;
}

pub fn send_msg_1040_1696(param_1: u32, param_2: HWND16) {
    let mut u_var1: u32;
    let p_uvar2: *mut u16;
    let mut u_var3: u16;
    let mut u_var4: u32;
    let mut extraout_dx: u16;
    let mut extraout_dx_00: i32;
    let mut extraout_dx_01: u16;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let w_var6: WPARAM16;
    let mut u_var7: u16;
    let mut local_22: u32;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u32;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    SendMessage16(0, 0, 0x40b, param_2);
    SendMessage16(0, 0, 0xb, param_2);
    local_4 = 0;
    pu_var2 = &local_4;
    u_var5 = (param_1 >> 0x10);
    pass1_1010_519a((param_1 + 0x8e), CONCAT22(unaff_ss, pu_var2));
    local_12 = 0;
    loop {
        if (local_4 <= local_12) {
            break;
        }
        u_var4 = (pu_var2 + local_12 * 2);
        w_var6 = 0;
        u_var7 = 0x403;
        u_var1 = (param_1 + 0x8e);
        str_fn_1010_5286(u_var1, (u_var1 >> 0x10), u_var4);
        SendMessage16(
            u_var4 & 0xffff | extraout_dx_00 << 0x10,
            w_var6,
            u_var7,
            param_2,
        );
        error_check_1000_17ce((u_var4 & 0xffff | extraout_dx_00 << 0x10));
        local_12 = local_12 + 1;
    }
    w_var6 = 0;
    u_var7 = 0x40a;
    u_var3 = local_4;
    load_string_1010_847e(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x531,
    );
    SendMessage16(CONCAT22(extraout_dx_01, u_var3), w_var6, u_var7, param_2);
    SendMessage16(0, 1, 0xb, param_2);
    return;
}

pub fn destroy_win_1040_13b2(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let paVar4: *mut Struct199;
    let mut extraout_dx: u16;
    let struct_a: *mut Struct199;
    let mut u_var5: i32;
    let struct_a_00: *mut Struct199;
    let mut i_var6: i32;
    let mut i_var7: i32;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut unaff_cs: u16;
    let mut unaff_ss: u16;
    let LVar10: LRESULT;
    let mut local_232: u16;
    let mut local_230: u16;
    let paStack556: *mut Struct199;
    let mut local_116: u32;
    let mut local_112: u32;
    let mut local_10e: u16;
    let mut local_10c: u16;
    let mut local_10a: u16;
    let mut local_108: u16;
    let mut local_106: u16;
    let mut local_104: u16;
    let mut local_102: u16;
    let local_100: [Struct199; 6];
    let mut local_ae: u16;
    let mut local_ac: u16;
    let local_aa: [Struct199; 6];
    let mut local_58: u16;
    let mut local_56: u16;
    let mut local_54: [u8; 82];

    i_var6 = param_1;
    u_var8 = (param_1 >> 0x10);
    if (param_2 != 0) {
        local_56 = GetDlgItem16(0xfd2, (i_var6 + 6));
        SendMessage16(CONCAT22(unaff_ss, local_54), 0x51, 0xd, local_56);
        local_58 = local_54;
        local_232._0_1_ = unaff_ss;
        local_232._1_1_ = (unaff_ss >> 8);
        pass1_fn_1000_3e2c(CONCAT13(local_232._1_1_, CONCAT12(local_232, local_58)));
        local_ac = GetDlgItem16((s_vrpal_bmp_1050_183a + 4), (i_var6 + 6));
        LVar10 = SendMessage16(0, 0, 0x407, local_ac);
        local_ae = LVar10;
        if (local_ae != 0xffff) {
            SendMessage16(CONCAT22(unaff_ss, local_aa), local_ae, 0x408, local_ac);
        }
        local_ac = GetDlgItem16((s_vrpal_bmp_1050_183a + 5), (i_var6 + 6));
        LVar10 = SendMessage16(0, 0, 0x407, local_ac);
        paVar4 = LVar10;
        local_ae = paVar4;
        if (paVar4 != 0xffff) {
            paStack556 = local_ac;
            LVar10 = SendMessage16(
                CONCAT13(local_232._1_1_, CONCAT12(local_232, local_100)),
                paVar4,
                0x408,
                local_ac,
            );
            paVar4 = LVar10;
        }
        paStack556 = (_g_struct_73_1050_14cc >> 0x10);
        load_string_1010_847e();
        _local_104 = CONCAT22(extraout_dx, paVar4);
        paStack556 = local_100;
        struct_a = local_aa;
        local_230._0_1_ = SUB21(struct_a, 0);
        pass1_1000_3d7a(local_230);
        if (paVar4 != 0x0) {
            paStack556 = _local_104;
            paVar4 = local_aa;
            local_230._0_1_ = SUB21(paVar4, 0);
            pass1_1000_3d7a(local_230);
            if (paVar4 != 0x0) {
                paStack556 = _local_104;
                paVar4 = local_100;
                pass1_1000_3d7a(0);
                if (paVar4 != 0x0) {
                    paStack556 = local_aa;
                    pass1_1010_531c((i_var6 + 0x8e), CONCAT22(unaff_ss, paStack556));
                    paVar4 = local_100;
                    paStack556 = paVar4;
                    pass1_1010_52fc((i_var6 + 0x8e), CONCAT22(unaff_ss, paVar4));
                    u_var1 = (i_var6 + 0x8e);
                    paStack556 = (u_var1 >> 0x10);
                    pass1_1010_5120(u_var1, local_58);
                    _local_10a = CONCAT22(local_108, paVar4);
                    if (paVar4 == 0x0) {
                        paStack556 = 0x1010;
                        local_10c = struct_a_00;
                        process_struct_1000_179c(0xb4, struct_a_00);
                        u_var5 = local_10c | paVar4;
                        local_10e = paVar4;
                        if (u_var5 == 0) {
                            paVar4 = 0x0;
                            u_var5 = 0;
                        } else {
                            paStack556 = 0x57b;
                            mixed_1040_8520(
                                CONCAT13((local_10c >> 8), CONCAT12(local_10c, paVar4)),
                                g_h_window,
                                0x30,
                                2,
                                0x57b,
                                0x7d2,
                            );
                        }
                        local_230._0_1_ = u_var5;
                        local_230._1_1_ = (u_var5 >> 8);
                        ppc_var2 = (CONCAT13(local_230._1_1_, CONCAT12(local_230, paVar4)) + 0x74);
                        paStack556 = paVar4;
                        ppc_var2();
                        return;
                    }
                    u_var3 = (i_var6 + 0x8e);
                    local_112 = (u_var3 + 0x12);
                    u_var3 = (i_var6 + 0x8e);
                    u_var9 = (u_var3 >> 0x10);
                    i_var7 = u_var3;
                    local_116 = (i_var7 + 0x16);
                    u_var3 = (i_var6 + 0x8e);
                    local_106 = (u_var3 + 10);
                    paStack556 = local_116;
                    pass1_1028_8d9e(
                        CONCAT22(unaff_ss, &stack0xfdd2),
                        local_106,
                        local_112,
                        local_116 & 0xffff | *(i_var7 + 0x18) << 0x10,
                    );
                    paStack556 = &stack0xfdd2;
                    pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_ss, paStack556));
                    paStack556 = &stack0xfdd2;
                    unaff_cs = SUB42(&PTR_LOOP_1050_1028, 0);
                    pass1_1028_8dec(CONCAT22(unaff_ss, paStack556));
                    // goto LAB_1040_1619;
                }
            }
        }
        paStack556 = 0x1000;
        unaff_cs = 0x1000;
        process_struct_1000_179c(0xb4, struct_a);
        u_var5 = struct_a | paVar4;
        local_10c = struct_a;
        local_10e = paVar4;
        if (u_var5 == 0) {
            paVar4 = 0x0;
            u_var5 = 0;
        } else {
            paStack556 = 0x57b;
            mixed_1040_8520(
                CONCAT13((struct_a >> 8), CONCAT12(struct_a, paVar4)),
                g_h_window,
                0x30,
                2,
                0x57b,
                0x755,
            );
        }
        _local_10a = CONCAT22(u_var5, paVar4);
        ppc_var2 = (*_local_10a + 0x74);
        paStack556 = paVar4;
        ppc_var2();
    }
    // LAB_1040_1619:
    paStack556 = unaff_cs;
    DestroyWindow16((i_var6 + 6));
    return;
}

pub fn post_win_msg_1040_0d5e(param_1: u32, param_2: i32) {
    if (param_2 != 0) {
        PostMessage16(0, 1, 0x111, (param_1 + 8));
    }
    return;
}

pub fn send_win_msg_1038_ed8a(param_1: u16, param_2: u32, param_3: u32) {
    let mut u_var1: i32;
    let mut u_var2: u16;
    let paVar3: *mut Struct493;
    let mut i_var4: i32;
    let mut u_var5: i32;
    let ppVar6: *mut pass1_struct_1;
    let lVar7: u32;
    let h_wnd: HWND16;
    let mut in_stack_0000ffe2: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_10: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    h_wnd = g_h_window;
    if (param_3._2_2_ != 0x1c8) {
        if (param_3._2_2_ == 0x1c9) {
            ppVar6 =
                process_struct_1010_20ba(_g_Struct372_1050_0ed0, CONCAT22(in_stack_0000ffe2, 0x2f));
            u_var2 = (ppVar6 >> 0x10);
            u_var1 = (ppVar6 + 0x20);
            u_var5 = (ppVar6 + 0x22);
            if ((u_var5 | u_var1) == 0) {
                return;
            }
            paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var1, u_var5);
            if ((u_var5 | paVar3) == 0) {
                return;
            }
            i_var4 = pass1_1030_5b00(CONCAT22(u_var5, paVar3));
            ppVar6 = process_struct_1010_20ba(
                _g_Struct372_1050_0ed0,
                CONCAT22(in_stack_0000ffe2, i_var4),
            );
            if (ppVar6 == 0x0) {
                return;
            }
            lVar7 = pass1_1018_0ad4(ppVar6);
            if (lVar7 == 0) {
                return;
            }
            param_3._2_2_ = 0x72;
            h_wnd = (lVar7 + 8);
        } else {
            if (param_3._2_2_ != 0x1ca) {
                post_win_msg_1040_7b3c(param_1, param_2, (param_2 >> 0x10), param_3, param_3._2_2_);
                return;
            }
        }
    }
    SendMessage16(0, param_3._2_2_, 0x111, h_wnd);
    return;
}

pub fn pass1_1038_e4bc(param_1: *mut u8, param_2: *mut u8, param_3: *mut u8) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let paVar3: *mut Struct493;
    let mut u_var4: i32;
    let pu_var5: *mut u8;
    let mut u_var6: i32;
    let mut u_var7: i32;
    let mut extraout_dx: u16;
    let mut extraout_dx_00: i32;
    let mut extraout_dx_01: u16;
    let mut u_var8: u16;
    let mut unaff_si: u16;
    let pp_var9: *mut pass1_struct_1;
    let mut u_var10: u16;
    let u_var11: u8;
    let mut u_var12: u16;
    let mut u_var13: u16;
    let mut u_var14: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_3._2_2_ == 0x1c4) {
        pp_var9 = process_struct_1010_20ba(_g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x2f));
        u_var12 = (pp_var9 >> 0x10);
        u_var7 = (pp_var9 + 0x24);
        u_var6 = (pp_var9 + 0x26);
        if ((u_var6 | u_var7) != 0) {
            paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var7, u_var6);
            u_var7 = u_var6 | paVar3;
            if (u_var7 != 0) {
                pu_var5 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x20);
                u_var4 = pu_var5;
                pass1_1038_4e78(CONCAT22(u_var6, paVar3), pu_var5 & 0xffff | u_var7 << 0x10);
                _local_16 = CONCAT22(extraout_dx, u_var4);
                u_var2 = *_local_16;
                pp_var1 = u_var2 + 8;
                u_var7 = u_var4;
                (**pp_var1)(&PTR_LOOP_1050_1008, u_var4, extraout_dx);
                if ((extraout_dx_00 | u_var7) == 0) {
                    if (_local_16 != 0x0) {
                        pp_var1 = u_var2;
                        (**pp_var1)(&PTR_LOOP_1050_1008, u_var4, extraout_dx, 1);
                    }
                } else {
                    pp_var1 = (*_local_16 + 4);
                    u_var6 = u_var4;
                    (**pp_var1)(8, u_var4, extraout_dx, 0, 0);
                    u_var8 = extraout_dx_01;
                    paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var7, extraout_dx_01);
                    pp_var9 =
                        process_struct_1010_20ba(_g_Struct372_1050_0ed0, CONCAT22(u_var6, 0x32));
                    win_fn_1010_71d6(
                        pp_var9,
                        1,
                        ((u_var8 & 0xff00) << 0x10 | CONCAT12(u_var8, &paVar3.field_0xc)),
                    );
                    if (_local_16 != 0x0) {
                        pp_var1 = *_local_16;
                        (**pp_var1)(0x1010, u_var4, extraout_dx, 1);
                    }
                }
            }
        }
    } else {
        if (param_3._2_2_ == 0x1c5) {
            u_var12 = 0xe;
        } else {
            if (param_3._2_2_ != 0x1c6) {
                post_win_msg_1040_7b3c(param_1, param_2, (param_2 >> 0x10), param_3);
                return;
            }
            u_var12 = 0xd;
        }
        u_var14 = 0;
        u_var13 = 0;
        u_var10 = 0;
        u_var11 = 0;
        pp_var9 = process_struct_1010_20ba(_g_Struct372_1050_0ed0, 0x32);
        window_msg_func_1010_7300(
            pp_var9,
            (pp_var9 >> 0x10),
            u_var10,
            u_var11,
            u_var12,
            u_var13,
            u_var14,
        );
    }
    return;
}

pub fn post_win_msg_1038_dcb0(param_1: u32) {
    let pp_var1: fn();
    let mut in_AX: i32;
    let mut u_var2: u16;
    let mut u_var3: u16;
    let in_dx: *mut Struct199;
    let mut u_var4: i32;
    let struct_a: *mut Struct199;
    let mut i_var5: i32;
    let mut unaff_ss: u16;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let u_var9: u8;
    let u_var10: u8;
    let mut u_var11: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: [u8; 4];
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 4];
    let mut local_6: u16;
    let mut local_4: u16;

    process_struct_1000_179c(0xb4, in_dx);
    u_var4 = in_dx | in_AX;
    i_var5 = param_1;
    u_var3 = (param_1 >> 0x10);
    if (u_var4 == 0) {
        u_var2 = 0;
        u_var4 = 0;
    } else {
        u_var2 = mixed_1040_8520(CONCAT22(in_dx, in_AX), (i_var5 + 6), 4, 3, 0x634, 0x726);
    }
    _local_6 = CONCAT22(u_var4, u_var2);
    pass1_1008_941a(CONCAT22(unaff_ss, local_a), 1, 0x49);
    pp_var1 = (*_local_6 + 0x6c);
    _local_10 = (**pp_var1)();
    struct_a = (_local_10 >> 0x10);
    local_c = _local_10;
    if (local_c == 6) {
        process_struct_1000_179c(0xb4, struct_a);
        u_var4 = (_local_10 >> 0x10) | _local_10;
        if (_local_10 == 0x0) {
            u_var3 = 0;
            u_var4 = 0;
        } else {
            u_var3 = mixed_1040_8520(_local_10, (i_var5 + 6), 0, 2, 0x634, 0x728);
        }
        _local_6 = CONCAT22(u_var4, u_var3);
        pass1_1008_941a(CONCAT22(unaff_ss, local_14), 1, 0x4a);
        pp_var1 = (*_local_6 + 0x6c);
        (**pp_var1)(
            &PTR_LOOP_1050_1008,
            _local_6,
            (_local_6 >> 0x10),
            local_14,
            unaff_ss,
        );
        u_var9 = 0;
        u_var10 = 0;
        u_var11 = 0x15;
        u_var7 = 1;
        u_var8 = 0;
        u_var2 = 0;
        u_var6 = 0;
        u_var3 = 0;
        _local_18 = process_struct_1010_20ba(_g_Struct372_1050_0ed0, 0x37);
        post_win_msg_1008_a0e4(
            _local_18,
            CONCAT22(u_var2, u_var3),
            u_var6,
            u_var7,
            CONCAT13(u_var10, CONCAT12(u_var9, u_var8)),
            u_var11,
        );
        PostMessage16(0, 0xfc, 0x111, g_h_window);
        return;
    }
    process_struct_1000_179c(0xb4, struct_a);
    u_var4 = (_local_10 >> 0x10) | _local_10;
    if (_local_10 == 0x0) {
        u_var3 = 0;
        u_var4 = 0;
    } else {
        u_var3 = mixed_1040_8520(_local_10, (i_var5 + 6), 0, 2, 0x634, 0x729);
    }
    _local_6 = CONCAT22(u_var4, u_var3);
    pass1_1008_941a(CONCAT22(unaff_ss, &local_18), 1, 0x4b);
    pp_var1 = (*_local_6 + 0x6c);
    (**pp_var1)(
        &PTR_LOOP_1050_1008,
        _local_6,
        (_local_6 >> 0x10),
        &local_18,
        unaff_ss,
    );
    return;
}

pub fn win_fn_1038_da68(param_1: i32, param_2: u16, param_3_00: i32, param_3: u32) {
    let pp_var1: fn();
    let u_var2: u8;
    let pu_var3: *mut u8;
    let extraout_var: u32;
    let in_dx: *mut Struct199;
    let extraout_dx: *mut Struct199;
    let mut u_var4: i32;
    let extraout_dx_00: *mut Struct199;
    let extraout_dx_01: *mut Struct199;
    let extraout_dx_02: *mut Struct199;
    let extraout_dx_03: *mut Struct199;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let puVar6: *mut u8;
    let mut u_var7: u16;
    let mut local_16: [u8; 4];
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    if (param_3_00 == 0x204) {
        bad1_1038_de20(
            CONCAT13((param_2 >> 8), CONCAT12(param_2, param_1)),
            (s_SOLDefaultWindowClass_1050_01fe + 6),
            param_3,
            param_3._2_2_,
        );
        return;
    }
    local_6 = 0;
    local_8 = 0;
    if (param_3._2_2_ == (s_New_failed_in_Op__Op__Simulator_1050_0110 + 0x11)) {
        local_a = 1;
        local_6 = 0x6ec0000;
        local_8 = 0x15;
    } else {
        if ((s_New_failed_in_Op__Op__Simulator_1050_0110 + 0x11) < param_3._2_2_) {
            if (param_3._2_2_ == (s_New_failed_in_Op__Op__Simulator_1050_0110 + 0x12)) {
                u_var7 = 0x14;
            } else {
                if (param_3._2_2_ != (s_New_failed_in_Op__Op__Simulator_1050_0110 + 0x13)) {
                    if (param_3._2_2_ + -0x125 == 0x0) {
                        pp_var1 = (*_g_struct_ptr_1050_02a0 + 4);
                        param_3._2_2_ = param_3._2_2_ + -0x125;
                        (**pp_var1)();
                        (param_1 + 0x90) = 1;
                        mci_send_cmd_1008_5c5c(_g_struct_ptr_1050_02a0, 0x1db);
                        (param_1 + 0x8e) = 0x100;
                        in_dx = extraout_dx_01;
                    } else {
                        pu_var3 = param_3._2_2_ + -0x126;
                        if (pu_var3 == 0x0) {
                            (param_1 + 0x8e) = 0;
                            mci_send_command_1008_5c7c(_g_struct_ptr_1050_02a0, 0xcb0001);
                            mixed_fn_1010_830a(_g_struct_73_1050_14cc, 0x1f8);
                            _local_e = CONCAT22(extraout_dx_02, pu_var3);
                            in_dx = extraout_dx_02;
                            param_3._2_2_ =
                                WinHelp16(0, 3, CONCAT22(extraout_dx_02, pu_var3), (param_1 + 6));
                        } else {
                            if (param_3._2_2_ + -0x127 != 0x0) {}
                            // goto LAB_1038_dc20;
                            param_3._2_2_ = param_3._2_2_ + -0x127;
                            post_win_msg_1038_dcb0(param_1, param_2);
                            in_dx = extraout_dx_03;
                        }
                    }
                    // goto LAB_1038_dac3;
                }
                u_var7 = 0x28;
            }
            // LAB_1038_db1c:
            puVar6 = pass1_1038_af40(_g_Struct112_a, *(param_1 + 8), u_var7);
            in_dx = (puVar6 >> 0x10);
            param_3._2_2_ = puVar6;
        } else {
            if (param_3._2_2_ + -0x100 == 0x0) {
                param_3._2_2_ = param_3._2_2_ + -0x100;
                if ((param_1 + 0x8e) == 0) {
                    u_var2 = pass1_1010_1ea6(_g_struct_ptr_1050_02a0, CONCAT22(param_2, param_1));
                    param_3._2_2_ = CONCAT31(extraout_var, u_var2);
                    (param_1 + 0x90) = 0;
                }
                local_6 = CONCAT22(0x72c, local_6);
                local_8 = 0x48;
            } else {
                if (param_3._2_2_ + -0x11c == 0x0) {
                    param_3._2_2_ = param_3._2_2_ + -0x11c;
                    pass1_1038_df86(CONCAT22(param_2, param_1));
                    in_dx = extraout_dx;
                } else {
                    if (param_3._2_2_ + -0x11d != 0x0) {
                        if (param_3._2_2_ == (s_New_failed_in_Op__Op__Simulator_1050_0110 + 0xe)) {
                            u_var7 = 0x1d;
                        } else {
                            if (param_3._2_2_
                                != (s_New_failed_in_Op__Op__Simulator_1050_0110 + 0x10))
                            {
                                // LAB_1038_dc20:
                                post_win_msg_1040_7b3c(
                                    param_1,
                                    param_2,
                                    param_3_00,
                                    param_3,
                                    param_3._2_2_,
                                );
                                return;
                            }
                            u_var7 = 0x1c;
                        }
                        // goto LAB_1038_db1c;
                    }
                    param_3._2_2_ = param_3._2_2_ + -0x11d;
                    pass1_1038_df5c(CONCAT22(param_2, param_1));
                    in_dx = extraout_dx_00;
                }
            }
        }
    }
    // LAB_1038_dac3:
    if (local_6._2_2_ == 0) {
        return;
    }
    if (local_6 == 0) {
        process_struct_1000_179c(0xb4, in_dx);
        u_var4 = in_dx | param_3._2_2_;
        local_12 = param_3._2_2_;
        local_10 = in_dx;
        if (u_var4 != 0) {
            u_var5 = SUB42(&PTR_LOOP_1050_1040, 0);
            mixed_1040_8520(
                CONCAT13((in_dx >> 8), CONCAT12(in_dx, param_3._2_2_)),
                (param_1 + 6),
                0,
                2,
                0x634,
                local_6._2_2_,
            );
            // goto LAB_1038_dc37;
        }
    } else {
        process_struct_1000_179c(0xb4, in_dx);
        u_var4 = in_dx | param_3._2_2_;
        local_12 = param_3._2_2_;
        local_10 = in_dx;
        if (u_var4 != 0) {
            u_var5 = SUB42(&PTR_LOOP_1050_1040, 0);
            mixed_1040_8520(
                CONCAT13((in_dx >> 8), CONCAT12(in_dx, param_3._2_2_)),
                (param_1 + 6),
                0,
                3,
                0x634,
                local_6._2_2_,
            );
            // goto LAB_1038_dc37;
        }
    }
    u_var5 = 0x1000;
    param_3._2_2_ = 0x0;
    u_var4 = 0;
    // LAB_1038_dc37:
    _local_e = CONCAT22(u_var4, param_3._2_2_);
    if (local_8 == 0) {
        pp_var1 = (*_local_e + 0x74);
        (**pp_var1)(u_var5, param_3._2_2_, u_var4);
    } else {
        pass1_1008_941a(CONCAT22(unaff_ss, local_16), 1, local_8);
        pp_var1 = (*_local_e + 0x6c);
        (**pp_var1)(&PTR_LOOP_1050_1008, _local_e, (_local_e >> 0x10), local_16);
    }
    return;
}

pub fn post_win_msg_1038_d840(param_1: *mut u8, uparam_2: i32) {
    let mut i_var1: i32;
    let mut u_var2: u16;

    i_var1 = param_1;
    u_var2 = (param_1 >> 0x10);
    if (param_2 == 0x10) {
        if ((i_var1 + 0x8e) != 0) {
            PostMessage16(0, (i_var1 + 0x8e), 0x111, (i_var1 + 6));
            (i_var1 + 0x8e) = 0;
            return;
        }
    } else {
        if (param_2 < 0x11) {
            if (param_2 == 0x1) {
                (i_var1 + 0x90) = 0;
                (i_var1 + 0x92) = 0;
                return;
            }
            if (param_2 == '\x03') {
                pass1_1038_e03e(param_1);
                return;
            }
        }
    }
    return;
}

pub fn win_fn_1038_d118(param_1: u32, param_2: u32, param_3: HWND16) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut cVar3: u8;
    let mut in_AX: u16;
    let h_var4: HANDLE16;
    let HVar5: HANDLE16;
    let mut u_var6: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    h_var4 = GetProp16(CONCAT22(in_AX, 0x5bf3), param_3);
    HVar5 = GetProp16(CONCAT22(in_AX, 0x5bec), param_3);
    _local_6 = CONCAT22(h_var4, HVar5);
    if (param_2._2_2_ == 0x30) {
        if (param_2 == 0) {
            return;
        }
        SetProp16(param_2, CONCAT22(in_AX, 0x5c06), param_3);
        return;
    }
    u_var6 = (param_1 >> 0x10);
    if (param_2._2_2_ < 0x31) {
        cVar3 = (param_2 >> 0x10);
        if (cVar3 == 0x2) {
            if ((h_var4 | HVar5) != 0) {
                u_var2 = *_local_6;
                pp_var1 = u_var2 + 6;
                (**pp_var1)(
                    offset,
                    HVar5,
                    h_var4,
                    param_1,
                    u_var6,
                    param_2,
                    param_2._2_2_,
                );
                if (_local_6 != 0x0) {
                    pp_var1 = u_var2;
                    (**pp_var1)(offset, HVar5, h_var4, 1);
                }
            }
            h_var4 = GetProp16(CONCAT22(in_AX, 0x5bfa), param_3);
            if (h_var4 == 0) {
                return;
            }
            DeleteObject16(h_var4);
            RemoveProp16(CONCAT22(in_AX, 0x5c00), param_3);
            return;
        }
        if (cVar3 == '\x06') {
            if ((param_2 != 1) && (param_2 != 2)) {
                u_var2 = &PTR_LOOP_1050_5bc8;
                (u_var2 + 8) = 0;
                return;
            }
            u_var2 = &PTR_LOOP_1050_5bc8;
            (u_var2 + 8) = param_3;
            return;
        }
    }
    if ((h_var4 | HVar5) != 0) {
        pp_var1 = (*_local_6 + 0xc);
        (**pp_var1)(
            offset,
            HVar5,
            h_var4,
            param_1,
            u_var6,
            param_2,
            param_2._2_2_,
        );
    }
    return;
}

pub fn call_win_proc_1038_d020(param_1: u16, param_2: u32, param_3: u32) {
    let pp_var1: fn();
    let mut in_AX: u16;
    let HVar2: HANDLE16;
    let win_proc: &mut Vec<u8>;
    let HVar3: HANDLE16;
    let mut u_var4: i32;
    let LVar5: LRESULT;
    let mut u_var6: u32;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    HVar2 = GetProp16(CONCAT22(in_AX, 0x5bd7), param_3._2_2_);
    win_proc = GetProp16(
        CONCAT13((in_AX >> 8), CONCAT12(in_AX, 0x5bd0)),
        param_3._2_2_,
    );
    _local_6 = CONCAT22(HVar2, win_proc);
    HVar2 = GetProp16(CONCAT22(in_AX, 0x5be5), param_3._2_2_);
    HVar3 = GetProp16(CONCAT22(in_AX, 0x5bde), param_3._2_2_);
    _local_a = CONCAT22(HVar2, HVar3);
    if ((HVar2 | HVar3) != 0) {
        local_e = 0;
        if (param_3 == 0x19) {
            pp_var1 = (*_local_a + 0x34);
            local_e = (**pp_var1)(offset, HVar3, HVar2, param_1, param_2);
        } else {
            if (param_3 == 0x86) {
                pp_var1 = (*_local_a + 0x20);
                u_var4 = (**pp_var1)(offset, HVar3, HVar2, param_2._2_2_);
                // goto LAB_1038_d10e;
            }
            if ((param_3 == 0x112) && ((param_2._2_2_ & 0xfff0) == 0xf140)) {
                LVar5 = SendMessage16(0, 0xf140, 0x112, &g_h_window);
                u_var4 = (LVar5 == 0);
                // goto LAB_1038_d10e;
            }
        }
        if (local_e != 0) {
            return local_e;
        }
    }
    if (_local_6 != 0) {
        u_var6 = CallWindowProc16(
            CONCAT22(param_2, param_1),
            param_2._2_2_,
            param_3,
            param_3._2_2_,
            win_proc,
        );
        return u_var6;
    }
    u_var4 = 0;
    // LAB_1038_d10e:
    return u_var4;
}

pub fn make_proc_inst_1038_cf6c(param_1: *mut u16) {
    let pu_var1: *mut u8;
    let pu_var2: *mut u8;
    let unaff_cs: HANDLE16;
    let pvVar3: &mut Vec<u8>;

    pu_var2 = (param_1 >> 0x10);
    pu_var1 = param_1;
    unsafe {
        *param_1 = s_1_1050_389a;
    }
    (pu_var1 + 2) = &PTR_LOOP_1050_1008;
    (pu_var1 + 4) = 0;
    (pu_var1 + 8) = 0;
    unsafe {
        *param_1 = 0xd23e;
    }
    (pu_var1 + 2) = &PTR_LOOP_1050_1038;
    PTR_LOOP_1050_5bc8 = pu_var1;
    PTR_LOOP_1050_5bca = pu_var2;
    pvVar3 = MakeProcInstance16(unaff_cs, CONCAT22(0xd116, g_h_instance_1050_038c));
    (pu_var1 + 4) = pvVar3;
    (pu_var1 + 6) = (pvVar3 >> 0x10);
    pvVar3 = MakeProcInstance16(offset, CONCAT22(0xd01e, g_h_instance_1050_038c));
    PTR_LOOP_1050_5bcc = pvVar3;
    PTR_LOOP_1050_5bce = (pvVar3 >> 0x10);
    return;
}

pub fn destroy_win_1038_cc00(param_1: i32, param_2: u16, param_3: u16, param_4: u32) {
    let mut i_var1: i32;
    let mut u_var2: u16;

    if (param_3._2_2_ == 0x1cd) {
        u_var2 = 1;
    } else {
        if (param_3._2_2_ == 0x1ce) {
            u_var2 = 2;
        } else {
            if (param_3._2_2_ == 0x1cf) {
                u_var2 = 3;
            } else {
                if (param_3._2_2_ == 0x1d0) {
                    u_var2 = 4;
                } else {
                    if (param_3._2_2_ != 0x1d1) {
                        post_win_msg_1040_7b3c(
                            param_1,
                            param_2,
                            param_3_00,
                            param_3,
                            param_3._2_2_,
                        );
                        return;
                    }
                    u_var2 = 5;
                }
            }
        }
    }
    i_var1 = pass1_1008_eb74((param_1 + 0x8e), u_var2);
    if (i_var1 != 0) {
        mci_send_command_1008_5c7c(_g_struct_ptr_1050_02a0, CONCAT22(i_var1, 1));
        DestroyWindow16((param_1 + 6));
    }
    return;
}

pub fn destroy_win_1038_c836(param_1: i32, param_2: u16, param_3: u16, param_4: u32) {
    let mut u_var1: u32;
    let mut unaff_ss: u16;
    let mut local_6: [u8; 4];

    if (param_2._2_2_ == 0xfce) {
        pass1_1008_941a(CONCAT22(unaff_ss, local_6), 1, 0xac);
        mci_send_command_1008_5c9e(_g_struct_ptr_1050_02a0, CONCAT22(unaff_ss, local_6));
        u_var1 = (param_1 + 0x8e);
        (u_var1 + 10) = 6;
        DestroyWindow16((param_1 + 6));
        PTR_LOOP_1050_5b80 = 0x0;
        return;
    }
    post_win_msg_1040_7b3c(param_1, CONCAT22(param_3, param_2_00), param_2);
    return;
}

pub fn win_fn_1038_c374(param_1: u32, param_2: *mut u32, param_3: HWND16) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut u_var3: u16;
    let mut u_var4: u32;
    let mut extraout_dx: u16;
    let mut extraout_dx_00: u16;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let LVar7: LRESULT;
    let in_struct_1: *mut Struct44;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var6 = SUB42(offset, 0);
    LVar7 = SendMessage16(0, 0, 0x40b, param_3);
    u_var3 = LVar7;
    u_var5 = (param_2 >> 0x10);
    unsafe {
        ppc_var2 = (*param_2 + 0x10);
    }
    ppc_var2(offset, param_2, u_var5);
    _local_6 = CONCAT22(extraout_dx, u_var3);
    local_a = 0;
    loop {
        if (_local_6 <= local_a) {
            return;
        }
        unsafe {
            ppc_var2 = (*param_2 + 4);
        }
        u_var4 = _local_6;
        ppc_var2(u_var6, param_2, u_var5, local_a, (local_a >> 0x10));
        u_var1 = (param_1 + 0x8e);
        in_struct_1 = process_struct_1008_e586(
            u_var1,
            (u_var1 >> 0x10),
            CONCAT13((extraout_dx_00 >> 8), CONCAT12(extraout_dx_00, u_var4)),
        );
        LVar7 = SendMessage16(in_struct_1, 0, 0x403, param_3);
        u_var6 = 0x1000;
        error_check_1000_17ce(in_struct_1);
        if (LVar7 == -1) {
            break;
        }
        if (LVar7 == -2) {
            return;
        }
        local_a = local_a + 1;
    }
    return;
}

pub fn send_win_msg_1038_c228(param_1: u32) -> LRESULT {
    let mut i_var1: i32;
    let mut u_var2: u16;
    let LVar3: LRESULT;
    let mut local_a: u16;
    let mut local_6: u16;

    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    LVar3 = SendMessage16(0, 0, 0x407, (i_var1 + 0x92));
    SendMessage16(0, 0, 0x407, (i_var1 + 0x94));
    SendMessage16(
        param_1 & 0xffff0000 | (i_var1 + 0x9e),
        LVar3,
        0x408,
        (i_var1 + 0x92),
    );
    LVar3 = SendMessage16(
        param_1 & 0xffff0000 | (i_var1 + 0x19e),
        0x408,
        0x408,
        (i_var1 + 0x94),
    );
    return LVar3;
}

pub fn check_gui_dlg_1038_b922(param_1: u32, param_2: u32, uparam_3: i32) {
    let pi_var1: *mut i32;
    let mut i_var2: i32;
    let in_struct_1: *mut Struct300;
    let ppc_var3: fn();
    let h_var4: HANDLE16;
    let mut u_var5: u16;
    let h_var6: HWND16;
    let mut u_var7: i32;
    let puVar8: *mut u8;
    let mut u_var9: u32;
    let pu_var10: *mut u16;
    let mut extraout_dx: i32;
    let extraout_dx_00: *mut Struct199;
    let struct_a: *mut Struct199;
    let mut extraout_dx_01: u16;
    let mut u_var11: u16;
    let mut i_var12: i32;
    let mut u_var13: u16;
    let mut u_var14: u16;
    let mut unaff_ss: u16;
    let LVar15: LRESULT;
    let uVar16: u8;
    let mut local_470: u16;
    let mut local_46e: u16;
    let mut local_46c: u16;
    let mut local_468: u16;
    let mut local_466: u16;
    let mut local_464: [u8; 80];
    let mut local_414: [u8; 1024];
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    local_6 = 0;
    i_var12 = param_1;
    u_var13 = (param_1 >> 0x10);
    if (param_3 < 0x1b5) {
        if (0x19f < param_3) {
            local_8 = GetDlgItem16(param_3, (i_var12 + 6));
            LVar15 = SendMessage16(0, 0, 0x400, local_8);
            local_a = LVar15;
            if (local_a == 2) {
                return;
            }
            SendMessage16(0, (local_a == 0), 0x401, local_8);
            u_var5 = IsDlgButtonChecked16(param_3, (i_var12 + 6));
            if (u_var5 != 0) {
                pi_var1 = (i_var12 + 0x96);
                unsafe {
                    *pi_var1 = *pi_var1 + -1;
                }
                h_var6 = GetDlgItem16(0xfb1, (i_var12 + 6));
                IsWindowEnabled16(offset, h_var6);
                u_var7 = extraout_dx;
                if (h_var6 == 0) {
                    h_var6 = GetDlgItem16(0xfb1, (i_var12 + 6));
                    EnableWindow16(1, h_var6);
                }
                if ((i_var12 + 0x96) < 0) {
                    CheckDlgButton16(0, (i_var12 + 0x98), (i_var12 + 6));
                    (i_var12 + 0x96) = 0;
                }
                (i_var12 + 0x98) = param_3;
                pass1_1018_1c9a(*(i_var12 + 0x92), param_3);
                u_var9 = pass1_1018_1e78((i_var12 + 0x92), 0xffff);
                _local_e = (u_var9 & 0xffff | u_var7 << 0x10);
                if ((u_var7 | u_var9) == 0) {
                    local_10 = 0;
                } else {
                    local_10 = (u_var9 + 0x1c);
                }
                mci_send_command_1008_5c7c(_g_struct_ptr_1050_02a0, CONCAT22(local_10, 1));
                return;
            }
            pi_var1 = (i_var12 + 0x96);
            unsafe {
                *pi_var1 = *pi_var1 + 1;
            }
            if ((i_var12 + 0x96) != 1) {
                return;
            }
            h_var6 = GetDlgItem16(0xfb1, (i_var12 + 6));
            EnableWindow16(0, h_var6);
            return;
        }
        if (param_3 == 2) {
            return;
        }
        // LAB_1038_bc20:
        uVar16 = param_3;
    } else {
        if (param_3 == 0xfb1) {
            local_46c._0_1_ = 0xa0;
            local_46c._1_1_ = 1;
            while (CONCAT11(local_46c._1_1_, local_46c) < 0x1b5) {
                u_var5 = IsDlgButtonChecked16(CONCAT11(local_46c._1_1_, local_46c), (i_var12 + 6));
                if (u_var5 == 1) {
                    in_struct_1 = (i_var12 + 0x8e);
                    uVar16 = (in_struct_1 >> 0x10);
                    switch_stmt_1008_d818(in_struct_1, CONCAT11(local_46c._1_1_, local_46c));
                    // goto LAB_1038_bba2;
                }
                i_var2 = CONCAT11(local_46c._1_1_, local_46c) + 1;
                local_46c._0_1_ = i_var2;
                local_46c._1_1_ = (i_var2 >> 8);
            }
        } else {
            if (param_3 != 0xfbe) {}
            // goto LAB_1038_bc20;
            _local_e = process_struct_1010_20ba(_g_Struct372_1050_0ed0, CONCAT22(local_470, 2));
            u_var7 = (_local_e >> 0x10);
            local_10 = u16_1050_13ae;
            if (u16_1050_13ae == 1) {
                local_10 = 2;
            }
            local_a = (local_10 * 0xc + 0x5b84) - 1;
            pass1_fn_1008_612e(0, local_a);
            u_var9 = pass1_1018_1e78((i_var12 + 0x92), ((local_10 * 6 + local_a) * 2 + 0x5b86));
            _local_14 = (u_var9 & 0xffff | u_var7 << 0x10);
            load_string_1010_84e0(
                _g_struct_73_1050_14cc,
                (_g_struct_73_1050_14cc >> 0x10),
                0x50,
                CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, local_464)),
                0x5f1,
            );
            load_string_1010_847e(
                _g_struct_73_1050_14cc,
                (_g_struct_73_1050_14cc >> 0x10),
                *_local_14,
            );
            struct_a = extraout_dx_00;
            u_var7 = wsprintf16(
                local_414,
                CONCAT13(0x5b, CONCAT12(0xc0, unaff_ss)),
                CONCAT13((local_464 >> 8), CONCAT12(local_464, 0x1050)),
            );
            u_var14 = 0x1000;
            process_struct_1000_179c(0xb4, struct_a);
            h_var4 = g_h_window;
            if ((struct_a | u_var7) == 0) {
                local_8 = 0;
                u_var11 = 0;
            } else {
                local_470 = _g_struct_73_1050_14cc;
                local_46e = (_g_struct_73_1050_14cc >> 0x10);
                puVar8 = local_414;
                load_string_1010_847e(local_470, local_46e, 0x7b);
                local_46c._0_1_ = extraout_dx_01;
                local_46c._1_1_ = (extraout_dx_01 >> 8);
                u_var14 = SUB42(&PTR_LOOP_1050_1040, 0);
                u_var11 = extraout_dx_01;
                pu_var10 = process_struct_1040_8478(
                    CONCAT13((puVar8 >> 8), CONCAT12(puVar8, 0x41)),
                    0x41,
                    CONCAT13(local_46c._1_1_, CONCAT12(local_46c, puVar8)),
                    CONCAT22(unaff_ss, local_414),
                    h_var4,
                );
                local_8 = pu_var10;
            }
            ppc_var3 = (CONCAT22(u_var11, local_8) + 0x74);
            (**ppc_var3)(u_var14, local_8);
            if (local_8 != 1) {
                return;
            }
            switch_stmt_1008_d818((i_var12 + 0x8e), (_local_14 + 0x1a));
            uVar16 = *(_local_14 + 0x1a);
            // LAB_1038_bba2:
            set_cursor_1038_bc30(param_1, u_var13, uVar16);
        }
        PostMessage16(0, 0xce, 0x111, g_h_window);
        uVar16 = 1;
    }
    post_win_msg_1040_7b3c(
        param_1,
        (param_1 >> 0x10),
        param_2,
        (param_2 >> 0x10),
        uVar16,
    );
    return;
}

pub fn send_msg_1038_a9fa(param_1: u32, param_2: i32) {
    let h_wnd: u16;
    let mut i_var1: i32;
    let mut u_var2: u16;
    let ppVar3: *mut pass1_struct_1;
    let LVar4: LRESULT;
    let mut in_stack_0000fff0: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2 != 0) {
        ppVar3 = process_struct_1010_20ba(_g_Struct372_1050_0ed0, CONCAT22(in_stack_0000fff0, 2));
        u_var2 = (param_1 >> 0x10);
        i_var1 = param_1;
        LVar4 = SendDlgItemMessage16(0, 0, 0x400, 0x116, (i_var1 + 6));
        if (LVar4 == 0) {
            LVar4 = SendDlgItemMessage16(0, 0, 0x400, 0x117, (i_var1 + 6));
            if (LVar4 == 0) {
                LVar4 = SendDlgItemMessage16(0, 0, 0x400, 0x118, (i_var1 + 6));
                if (LVar4 == 0) {
                    LVar4 = SendDlgItemMessage16(0, 0, 0x400, 0x119, (i_var1 + 6));
                    if (LVar4 != 0) {
                        u16_1050_13ae = 4;
                    }
                } else {
                    u16_1050_13ae = 3;
                }
            } else {
                u16_1050_13ae = 2;
            }
        } else {
            u16_1050_13ae = 1;
        }
        LVar4 = SendDlgItemMessage16(0, 0, 0x400, 0x11a, (i_var1 + 6));
        (ppVar3 + 0x82) = LVar4;
        h_wnd = GetWindowWord16(-8, (i_var1 + 6));
        PostMessage16(0, 0x105, 0x111, h_wnd);
        destroy_win_1040_7b98(i_var1, u_var2, 1);
    }
    return;
}

pub fn pass1_1038_8d7e(param_1: *mut u8) {
    ret_1040_78de();
    win_fn_1038_8f74(param_1);
    return;
}

pub fn get_dlg_item_int_1038_8afe(param_1: u32) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let unaff_ss: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let local_4: bool;

    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    u_var2 = GetDlgItemInt16(0, &local_4, unaff_ss, (s_dibtext_bmp_1050_1844 + 9));
    pass1_1030_6c1a((i_var3 + 0x94), u_var2);
    u_var1 = (i_var3 + 0x94);
    pass1_1038_387e((u_var1 + 0x2e), u_var2, (i_var3 + 0x9c), (i_var3 + 0x94));
    return;
}

pub fn pass1_1038_8810(param_1: *mut u8) {
    let mut i_var1: i32;
    let mut unaff_ss: u16;
    let mut local_102: [u8; 256];

    i_var1 = send_dlg_item_msg_1038_8164(
        param_1,
        CONCAT22(unaff_ss, local_102),
        (s_logo_bmp_1050_1850 + 6),
    );
    if (i_var1 != 0) {
        pass1_1008_b63a(*(param_1 + 0x94), CONCAT22(unaff_ss, local_102));
    }
    return;
}

pub fn send_dlg_item_msg_1038_87b2(param_1: *mut u8) -> i32 {
    let mut u_var1: u32;
    let l_param: LPARAM;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut unaff_ss: u16;
    let LVar4: LRESULT;
    let mut local_106: u16;
    let mut local_104: u16;
    let mut local_102: [u8; 256];

    i_var2 = send_dlg_item_msg_1038_8164(
        param_1,
        CONCAT22(unaff_ss, local_102),
        (s_logo_bmp_1050_1850 + 5),
    );
    if (i_var2 != 0) {
        u_var3 = (param_1 >> 0x10);
        i_var2 = param_1;
        pass1_1008_b61a((i_var2 + 0x94), CONCAT22(unaff_ss, local_102));
        u_var1 = (i_var2 + 0x94);
        l_param = load_string_1008_b1f0(u_var1, (u_var1 >> 0x10));
        LVar4 = SendDlgItemMessage16(
            l_param,
            0xffff,
            0x40d,
            (s_logo_bmp_1050_1850 + 6),
            (i_var2 + 6),
        );
        i_var2 = LVar4;
    }
    return i_var2;
}

pub fn send_dlg_item_msg_1038_8618(param_1: *mut u8) -> i32 {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let lVar5: u32;
    let mut u_var6: u32;
    let LVar7: LRESULT;
    let mut local_112: u16;
    let mut local_110: u16;
    let mut local_10e: u16;
    let mut local_10c: u16;
    let mut local_10a: u16;
    let mut local_108: u16;
    let mut local_106: [u8; 256];
    let mut local_6: u16;
    let mut local_4: u16;

    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    _local_6 = pass1_1008_b820((i_var3 + 0x94));
    i_var2 = _local_6;
    if (_local_6 != 0) {
        i_var2 = send_dlg_item_msg_1038_8164(
            param_1,
            CONCAT22(unaff_ss, local_106),
            (s_logo_bmp_1050_1850 + 4),
        );
        if (i_var2 != 0) {
            SendDlgItemMessage16(0, 0, 0xb, (s_logo_bmp_1050_1850 + 5), (i_var3 + 6));
            SendDlgItemMessage16(0, 0, 0xb, (s_logo_bmp_1050_1850 + 6), (i_var3 + 6));
            SendDlgItemMessage16(0, 0, 0x405, (s_logo_bmp_1050_1850 + 5), (i_var3 + 6));
            SendDlgItemMessage16(0, 0, 0x405, (s_logo_bmp_1050_1850 + 6), (i_var3 + 6));
            u_var1 = (i_var3 + 0x94);
            lVar5 = pass1_1008_b4a0(u_var1, (u_var1 >> 0x10), local_106, unaff_ss);
            pass1_1008_b200((i_var3 + 0x94));
            if (lVar5 != 0) {
                send_dialog_item_msg_1038_8400(i_var3, u_var4, lVar5, (s_logo_bmp_1050_1850 + 5));
                u_var6 = pass1_1008_b366((i_var3 + 0x94));
                if (u_var6 != 0) {
                    SendDlgItemMessage16(
                        u_var6,
                        0xffff,
                        0x40d,
                        (s_logo_bmp_1050_1850 + 5),
                        (i_var3 + 6),
                    );
                }
            }
            u_var6 = pass1_1008_b38c((i_var3 + 0x94));
            if (u_var6 != 0) {
                send_dialog_item_msg_1038_8400(i_var3, u_var4, u_var6, (s_logo_bmp_1050_1850 + 6));
                u_var6 = pass1_1008_b47a((i_var3 + 0x94));
                if (u_var6 != 0) {
                    SendDlgItemMessage16(
                        u_var6,
                        0xffff,
                        0x40d,
                        (s_logo_bmp_1050_1850 + 6),
                        (i_var3 + 6),
                    );
                }
            }
            SendDlgItemMessage16(0, 1, 0xb, (s_logo_bmp_1050_1850 + 5), (i_var3 + 6));
            LVar7 = SendDlgItemMessage16(0, 1, 0xb, (s_logo_bmp_1050_1850 + 6), (i_var3 + 6));
            i_var2 = LVar7;
        }
    }
    return i_var2;
}

pub fn send_dialog_item_msg_1038_8400(param_1: u32, param_2: u32, param_3: u16) {
    let mut unaff_ss: u16;
    let lVar1: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(unaff_ss, local_a), param_2);
    while (true) {
        lVar1 = pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
        if (lVar1 == 0) {
            break;
        }
        SendDlgItemMessage16((lVar1 + 4), 0, 0x401, param_3, (param_1 + 6));
    }
    return;
}

pub fn pass1_1038_7dac(param_1: *mut u8) {
    ret_1040_78de();
    send_dialog_item_msg_1038_844a(param_1);
    return;
}

pub fn pass1_1038_7356(param_1: *mut Struct1159, param_2: *mut Struct921) {
    let pu_var1: *mut u32;
    let mut u_var2: u32;
    let in_struct_1: *mut Struct44;
    let lVar3: u32;
    let b_var4: bool;
    let mut u_var5: i32;
    let local_AX_243: *mut Struct1165;
    let paVar6: *mut Struct493;
    let mut u_var8: i32;
    let mut extraout_dx: i32;
    let mut extraout_dx_00: i32;
    let local_bx_40: *mut Struct1164;
    let local_bx_185: *mut Struct1166;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut bVar11: bool;
    let mut u_var12: u32;
    let mut u_var13: u32;
    let mut local_32: u32;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u32;
    let mut local_12: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let pu_var7: *mut u8;

    u_var12 = pass1_1030_73a8(param_2);
    u_var8 = (u_var12 >> 0x10);
    u_var5 = u_var8;
    b_var4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (u_var12 + 0xc), 4);
    local_bx_40 = param_1;
    u_var9 = (param_1 >> 0x10);
    if (b_var4 == 0) {
        b_var4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (u_var12 + 0xc), 3);
        if (b_var4 == 0) {
            // code_r0x10387545:
            pass1_1038_6f5a(param_1, param_2);
            // goto LAB_1038_7549;
        }
        if ((local_bx_40.field_0xc != 0) || (&local_bx_40.field_0xe != 0)) {
            u_var13 = pass1_1028_45e2(u_var12);
            u_var5 = (u_var13 >> 0x10);
            pu_var1 = &local_bx_40.field_0x18;
            unsafe {
                bVar11 = *pu_var1 < u_var5;
                if ((bVar11 || *pu_var1 == u_var5)
                    && (bVar11
                        || (
                            pu_var1 = &local_bx_40.field_0x16,
                            *pu_var1 < u_var13 || *pu_var1 == u_var13,
                        )))
                {}
            }
            // goto code_r0x10387545;
        }
    } else {
        u_var13 = pass1_1028_62c8(u_var12);
        u_var5 = (u_var13 >> 0x10);
        pu_var1 = &local_bx_40.field_0x18;
        unsafe {
            bVar11 = *pu_var1 < u_var5;
        }
        if ((bVar11 || unsafe { *pu_var1 == u_var5 })
            && (bVar11
                || (
                    pu_var1 = &local_bx_40.field_0x16,
                    unsafe { *pu_var1 < u_var13 } || unsafe { *pu_var1 == u_var13 },
                )))
        {
            if (local_bx_40.field_0x12 == 0) {
                if (local_bx_40.field_0x14 == 0) {}
                // goto LAB_1038_74e0;
                pu_var7 = _PTR_LOOP_1050_68a2;
                alloc_mem_1000_07fc(_PTR_LOOP_1050_68a2);
                u_var5 = pu_var7;
                local_32 = pu_var7 & 0xffff | extraout_dx_00 << 0x10;
                if ((extraout_dx_00 | u_var5) == 0) {
                    local_32 = 0;
                } else {
                    local_32 = s_1_1050_389a;
                    (u_var5 + 2) = &PTR_LOOP_1050_1008;
                    (u_var5 + 4) = 0;
                    (u_var5 + 6) = 0;
                    (u_var5 + 8) = 0;
                    (u_var5 + 10) = 0;
                    (u_var5 + 0xc) = 0;
                    local_32 = 0x56ce;
                    (u_var5 + 2) = 0x1018;
                }
                u_var10 = (local_32 >> 0x10);
                local_bx_185 = local_32;
                local_bx_185.field_0x8 = local_bx_40.field_0x14;
                local_bx_185.field_0xa = local_bx_40.field_0x16;
                u_var5 = pass1_1020_c42e(local_bx_40.field_0x14);
            } else {
                pu_var7 = _PTR_LOOP_1050_68a2;
                alloc_mem_1000_07fc(_PTR_LOOP_1050_68a2);
                u_var5 = pu_var7;
                local_1a = pu_var7 & 0xffff | extraout_dx << 0x10;
                if ((extraout_dx | u_var5) == 0) {
                    local_1a = 0;
                } else {
                    local_1a = s_1_1050_389a;
                    (u_var5 + 2) = &PTR_LOOP_1050_1008;
                    (u_var5 + 4) = 0;
                    (u_var5 + 6) = 0;
                    (u_var5 + 8) = 0;
                    (u_var5 + 10) = 0;
                    (u_var5 + 0xc) = 0;
                    local_1a = 0x56ce;
                    (u_var5 + 2) = 0x1018;
                }
                u_var10 = (local_1a >> 0x10);
                local_bx_185 = local_1a;
                local_bx_185.field_0x6 = local_bx_40.field_0x12;
                local_bx_185.field_0xa = local_bx_40.field_0x16;
                u_var5 = switch_statement_1020_c3b4(local_bx_40.field_0x12);
            }
            lVar3 = u_var5 * local_bx_185.field_0xa;
            u_var5 = (lVar3 >> 0x10);
            local_bx_185.field_0xc = lVar3;
            pass1_1028_6408(u_var12, CONCAT22(u_var10, local_bx_185));
            // goto LAB_1038_7549;
        }
    }
    // LAB_1038_74e0:
    pass1_1038_709c(param_1, param_2);
    // LAB_1038_7549:
    u_var2 = local_bx_40.field_0x8;
    paVar6 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var2, (u_var2 >> 0x10));
    pass1_1030_6c4c(
        CONCAT22(u_var5, paVar6),
        &paVar6[1].field_0x16 + local_bx_40.field_0x26,
    );
    local_bx_40.field_0xc = 0;
    local_bx_40.field_0x12 = 0;
    local_bx_40.field_0x14 = 0;
    &local_bx_40.field_0x16 = 0;
    in_struct_1 = &local_bx_40.field_0xe;
    u_var5 = local_bx_40.field_0x10;
    if ((u_var5 | in_struct_1) != 0) {
        pass1_1020_ba7e((in_struct_1 & 0xffff | u_var5 << 0x10));
        error_check_1000_17ce(in_struct_1);
    }
    &local_bx_40.field_0xe = 0;
    return;
}

pub fn pass1_1038_709c(param_1: *mut Struct1159, param_2: *mut Struct921) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let local_AX_152: *mut Struct1162;
    let mut u_var3: i32;
    let local_AX_247: *mut Struct1158;
    let local_AX_399: *mut Struct1160;
    let mut i_var4: i32;
    let pu_var5: *mut u8;
    let mut extraout_dx: i32;
    let mut extraout_dx_00: i32;
    let mut extraout_dx_01: i32;
    let mut extraout_dx_02: i32;
    let local_bx_4: *mut Struct1159;
    let local_bx_49: *mut Struct1163;
    let local_bx_337: *mut Struct1161;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let mut local_28: u32;
    let mut local_20: u32;
    let mut local_18: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u32;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var6 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    if ((local_bx_4.field_0x10 | &local_bx_4.field_0xe) == 0) {
        if (local_bx_4.field_0xc == 0) {
            if (local_bx_4.field_0x12 == 0) {
                if (local_bx_4.field_0x14 == 0) {
                    return;
                }
                pu_var5 = _PTR_LOOP_1050_68a2;
                alloc_mem_1000_07fc(_PTR_LOOP_1050_68a2);
                u_var3 = pu_var5;
                local_28 = pu_var5 & 0xffff | extraout_dx_02 << 0x10;
                if ((extraout_dx_02 | u_var3) == 0) {
                    local_28 = 0;
                } else {
                    local_28 = s_1_1050_389a;
                    (u_var3 + 2) = &PTR_LOOP_1050_1008;
                    (u_var3 + 4) = 0;
                    (u_var3 + 6) = 0;
                    (u_var3 + 8) = 0;
                    (u_var3 + 10) = 0;
                    (u_var3 + 0xc) = 0;
                    local_28 = 0x56ce;
                    (u_var3 + 2) = 0x1018;
                }
                u_var7 = (local_28 >> 0x10);
                (local_28 + 8) = local_bx_4.field_0x14;
                (local_28 + 10) = local_bx_4.field_0x16;
                u_var2 = pass1_1020_c42e(local_bx_4.field_0x14);
            } else {
                pass1_1030_7c50(param_2, &local_bx_4.field_0x16, local_bx_4.field_0x12);
                pu_var5 = _PTR_LOOP_1050_68a2;
                alloc_mem_1000_07fc(_PTR_LOOP_1050_68a2);
                u_var3 = pu_var5;
                local_28 = pu_var5 & 0xffff | extraout_dx_01 << 0x10;
                if ((extraout_dx_01 | u_var3) == 0) {
                    local_28 = 0;
                } else {
                    local_28 = s_1_1050_389a;
                    (u_var3 + 2) = &PTR_LOOP_1050_1008;
                    (u_var3 + 4) = 0;
                    (u_var3 + 6) = 0;
                    (u_var3 + 8) = 0;
                    (u_var3 + 10) = 0;
                    (u_var3 + 0xc) = 0;
                    local_28 = 0x56ce;
                    (u_var3 + 2) = 0x1018;
                }
                u_var7 = (local_28 >> 0x10);
                (local_28 + 6) = local_bx_4.field_0x12;
                (local_28 + 10) = local_bx_4.field_0x16;
                u_var2 = switch_statement_1020_c3b4(local_bx_4.field_0x12);
            }
            u_var7 = (local_28 >> 0x10);
            local_bx_337 = local_28;
            i_var4 = u_var2 * local_bx_337.field_0xa;
        } else {
            pu_var5 = _PTR_LOOP_1050_68a2;
            alloc_mem_1000_07fc(_PTR_LOOP_1050_68a2);
            u_var3 = pu_var5;
            local_28 = pu_var5 & 0xffff | extraout_dx_00 << 0x10;
            if ((extraout_dx_00 | u_var3) == 0) {
                local_28 = 0;
            } else {
                local_28 = s_1_1050_389a;
                (u_var3 + 2) = &PTR_LOOP_1050_1008;
                (u_var3 + 4) = 0;
                (u_var3 + 6) = 0;
                (u_var3 + 8) = 0;
                (u_var3 + 10) = 0;
                (u_var3 + 0xc) = 0;
                local_28 = 0x56ce;
                (u_var3 + 2) = 0x1018;
            }
            u_var7 = (local_28 >> 0x10);
            local_bx_337 = local_28;
            local_bx_337.field_0x4 = local_bx_4.field_0xc;
            i_var4 = local_bx_4.field_0x16;
            local_bx_337.field_0xa = i_var4;
        }
        local_bx_337.field_0xc = i_var4;
        pass1_1030_6a2c(param_2, CONCAT22(u_var7, local_bx_337));
    } else {
        u_var1 = &local_bx_4.field_0xe;
        local_4 = (u_var1 + 4);
        local_c = 0;
        while (local_c < local_4) {
            pass1_1020_bb16(
                &local_bx_4.field_0xe,
                CONCAT22(unaff_ss, &local_a),
                CONCAT22(unaff_ss, &local_6),
                local_c,
            );
            if (local_a != 0) {
                pu_var5 = _PTR_LOOP_1050_68a2;
                alloc_mem_1000_07fc(_PTR_LOOP_1050_68a2);
                u_var3 = pu_var5;
                local_10 = pu_var5 & 0xffff | extraout_dx << 0x10;
                if ((extraout_dx | u_var3) == 0) {
                    local_10 = 0;
                } else {
                    local_10 = s_1_1050_389a;
                    (u_var3 + 2) = &PTR_LOOP_1050_1008;
                    (u_var3 + 4) = 0;
                    (u_var3 + 6) = 0;
                    (u_var3 + 8) = 0;
                    (u_var3 + 10) = 0;
                    (u_var3 + 0xc) = 0;
                    local_10 = 0x56ce;
                    (u_var3 + 2) = 0x1018;
                }
                u_var7 = (local_10 >> 0x10);
                local_bx_49 = local_10;
                local_bx_49.field_0x4 = local_6;
                local_bx_49.field_0xa = local_a;
                u_var2 = ret_one_1020_c3ae();
                local_bx_49.field_0xc = u_var2 * local_bx_49.field_0xa;
                pass1_1030_6a2c(param_2, local_10);
            }
            local_c = local_c + 1;
        }
    }
    return;
}

pub fn pass1_1038_6f5a(param_1: u32, param_2: *mut Struct921) {
    // ppu_var1: *mut *mut u8;
    let local_AX_168: *mut Struct1157;
    let mut u_var2: i32;
    let mut u_var3: u16;
    let mut extraout_dx: i32;
    let local_bx_4: *mut Struct1156;
    let mut i_var5: i32;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let mut local_18: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u32;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;
    let pu_var4: *mut u8;

    u_var6 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    if (local_bx_4.field_0xe == 0x0) {
        if (local_bx_4.field_0xc != 0) {
            pass1_1030_7ddc(param_2, local_bx_4.field_0x16, local_bx_4.field_0xc);
            return;
        }
        if (local_bx_4.field_0x12 != 0) {
            pass1_1030_7c50(param_2, local_bx_4.field_0x16, local_bx_4.field_0x12);
            return;
        }
        pu_var4 = _PTR_LOOP_1050_68a2;
        alloc_mem_1000_07fc(_PTR_LOOP_1050_68a2);
        u_var2 = pu_var4;
        local_10 = pu_var4 & 0xffff | extraout_dx << 0x10;
        if ((extraout_dx | u_var2) == 0) {
            local_10 = 0;
        } else {
            local_10 = s_1_1050_389a;
            (u_var2 + 2) = &PTR_LOOP_1050_1008;
            (u_var2 + 4) = 0;
            (u_var2 + 6) = 0;
            (u_var2 + 8) = 0;
            (u_var2 + 10) = 0;
            (u_var2 + 0xc) = 0;
            local_10 = 0x56ce;
            (u_var2 + 2) = 0x1018;
        }
        u_var7 = (local_10 >> 0x10);
        i_var5 = local_10;
        (i_var5 + 8) = local_bx_4.field_0x14;
        (i_var5 + 10) = &local_bx_4.field_0x16;
        u_var3 = pass1_1020_c42e(local_bx_4.field_0x14);
        (i_var5 + 0xc) = u_var3 * (i_var5 + 10);
        pass1_1030_6a2c(param_2, local_10);
    } else {
        ppu_var1 = local_bx_4.field_0xe;
        local_4 = (ppu_var1 + 4);
        local_c = 0;
        while (local_c < local_4) {
            pass1_1020_bb16(
                local_bx_4.field_0xe,
                CONCAT22(unaff_ss, &local_a),
                CONCAT22(unaff_ss, &local_6),
                local_c,
            );
            if (CONCAT22(local_a._2_2_, local_a) != 0) {
                pass1_1030_7ddc(param_2, CONCAT22(local_a._2_2_, local_a), local_6);
            }
            local_c = local_c + 1;
        }
    }
    return;
}

pub fn win_fn_1038_362e(param_1: *mut Struct1090) {
    let mut i_var1: i32;
    let local_bx_4: *mut Struct1090;
    let mut u_var2: i32;
    let ppVar3: *mut pass1_struct_1;
    let mut in_stack_0000fff8: u32;
    let mut u_var4: u16;

    u_var2 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    if (local_bx_4.field_0x214 == 0) {
        u_var4 = (in_stack_0000fff8 >> 0x10);
        i_var1 = pass1_1038_4f54((param_1 & 0xffff | u_var2 << 0x10), 0x1f);
        if (i_var1 == 0) {
            local_bx_4.field_0x214 = 0x14;
        } else {
            local_bx_4.field_0x214 = 0x28;
        }
        ppVar3 = process_struct_1010_20ba(_g_Struct372_1050_0ed0, CONCAT22(u_var4, 0x37));
        post_win_msg_1008_a0e4(ppVar3, 0, 0, 1, local_bx_4.field_0x4, 0x38);
        local_bx_4.field_0x216 = 0;
    }
    return;
}

pub fn pass1_1038_095e(param_1: u16, param_2: u16, param_1_00: i32, param_2_00: *mut Struct1115) {
    let pp_var1: fn();
    let paVar2: *mut Struct493;
    let mut bVar3: bool;
    let pu_var4: *mut u8;
    let pu_var5: *mut u8;
    let mut u_var6: u32;
    let mut u_var7: i32;
    let mut u_var8: u32;
    let mut u_var9: u16;
    let mut unaff_ss: u16;
    let mut u_var10: i32;
    let mut u_var11: u16;
    let mut local_48: u32;
    let mut local_44: u32;
    let mut local_40: u16;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: u32;
    let mut local_36: u32;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u32;
    let mut local_28: [u8; 2];
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = process_struct_1010_20ba(_g_Struct372_1050_0ed0, CONCAT22(local_48, 0x37));
    local_a = *_PTR_LOOP_1050_65e2;
    u_var9 = (param_2_00 >> 0x10);
    if (local_a % 10 == 0) {
        if (param_1_00 < 0xc9) {
            u_var11 = 0x3f;
        } else {
            if (param_1_00 < 800) {}
            // goto LAB_1038_09c3;
            u_var11 = 0x3e;
        }
        post_win_msg_1008_a0e4(_local_6, 0, 0, 1, (param_2_00 + 4), u_var11);
    }
    // LAB_1038_09c3:
    local_c = (param_2_00 + 0x22);
    local_e = 0;
    local_12 = *_PTR_LOOP_1050_65e2;
    u_var7 = (_PTR_LOOP_1050_65e2 + 2);
    if (local_c < 0x4b) {
        if (local_c < 0x3c) {
            if (local_c < 0x32) {}
            // goto LAB_1038_0a1c;
            u_var10 = 0x1e;
        } else {
            u_var10 = 0xf;
        }
    } else {
        u_var10 = 5;
    }
    u_var8 = (local_12 & 0xffff | u_var7 << 0x10) % u_var10;
    u_var7 = u_var8;
    if (u_var8 == 0) {
        local_e = 1;
    }
    // LAB_1038_0a1c:
    if (local_e != 0) {
        pu_var5 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0xf);
        u_var9 = SUB42(pu_var5, 0);
        pass1_1038_4e78(param_2_00, pu_var5 & 0xffff | u_var7 << 0x10);
        _local_16 = CONCAT22(u_var7, u_var9);
        pu_var5 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x1a);
        u_var11 = pu_var5;
        local_1a = u_var11;
        local_18 = u_var7;
        pass1_1038_4d6e(param_2_00, pu_var5 & 0xffff | u_var7 << 0x10);
        _local_1e = CONCAT22(u_var7, u_var11);
        pp_var1 = (*_local_16 + 0x10);
        (**pp_var1)(&PTR_LOOP_1050_1008, _local_16, (_local_16 >> 0x10));
        _local_22 = CONCAT22(u_var7, u_var11);
        pp_var1 = (*_local_1e + 0x10);
        (**pp_var1)(&PTR_LOOP_1050_1008, _local_1e, (_local_1e >> 0x10));
        _local_26 = CONCAT22(u_var7, u_var11);
        u_var8 = pass1_1030_bcae(local_28, unaff_ss);
        local_36 = 0;
        while (true) {
            u_var8 = u_var8 >> 0x10;
            u_var9 = 0x1030;
            if (_local_22 <= local_36) {
                break;
            }
            u_var6 = _local_22;
            pass1_1030_1d58(_local_16);
            paVar2 = (u_var6 & 0xffff | u_var8 << 0x10);
            bVar3 = false;
            local_3a = 0;
            while (local_3a < _local_26) {
                u_var6 = _local_26;
                pass1_1030_1d58(_local_1e);
                pu_var4 = local_28;
                pass1_1030_bd74(
                    pu_var4,
                    unaff_ss,
                    paVar2,
                    (u_var6 & 0xffff | u_var8 << 0x10),
                );
                if (pu_var4 < 6) {
                    bVar3 = true;
                    break;
                }
                local_3a = local_3a + 1;
            }
            u_var8 = pass1_1030_73a8(paVar2);
            if (!bVar3) {
                u_var9 = SUB42(&PTR_LOOP_1050_1028, 0);
                pass1_1028_5ca0(u_var8);
                break;
            }
            local_36 = local_36 + 1;
        }
        if (_local_16 != 0x0) {
            pp_var1 = *_local_16;
            (**pp_var1)(u_var9, _local_16, (_local_16 >> 0x10), 1);
        }
        if (_local_1e != 0x0) {
            pp_var1 = *_local_1e;
            (**pp_var1)(u_var9, _local_1e, (_local_1e >> 0x10), 1);
        }
    }
    return;
}

pub fn win_fn_1030_e67c(param_1: u32) -> u16 {
    let mut u_var1: u16;
    let ppVar2: *mut pass1_struct_1;
    let mut in_stack_0000fff6: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar2 = process_struct_1010_20ba(
        _g_Struct372_1050_0ed0,
        CONCAT22((in_stack_0000fff6 >> 0x10), 0x37),
    );
    u_var1 = switch_stmt_1008_aaa8(ppVar2, (ppVar2 >> 0x10), (param_1 + 0x108));
    if (u_var1 != 0) {
        post_win_msg_1008_a0e4(ppVar2, 0, 0, 1, 0, u_var1);
    }
    return 1;
}

pub fn pass1_1030_838e(param_1: *mut u32) {
    unsafe {
        pass1_1028_d2b0(*param_1, (*param_1 >> 0x10));
    }
    pass1_1028_d01a((param_1 + 4));
    send_win_msg_1028_e242(_PTR_LOOP_1050_65e2, (_PTR_LOOP_1050_65e2 >> 0x10));
    return;
}

pub fn pass1_1030_83ba(param_1: *mut u32, param_2: libc::c_long) {
    let lVar1: u32;

    while (lVar1 = param_2 + -1, param_2 != 0) {
        unsafe {
            pass1_1028_d2b0(*param_1, (*param_1 >> 0x10));
        }
        pass1_1028_d01a((param_1 + 4));
        param_2 = lVar1;
        if (lVar1 != 0) {
            send_win_msg_1028_e242(_PTR_LOOP_1050_65e2, (_PTR_LOOP_1050_65e2 >> 0x10), 0);
        }
    }
    send_win_msg_1028_e242(_PTR_LOOP_1050_65e2, (_PTR_LOOP_1050_65e2 >> 0x10), 1);
    return;
}

pub fn send_win_msg_1028_e242(param_1: *mut u32, param_2: i32) {
    if (unsafe { *param_1 % 100 == 0 }) {
        SendMessage16(0, 0, 0x41, g_h_window);
    }
    unsafe {
        *param_1 = *param_1 + 1;
    }
    if (param_2 != 0) {
        bad_1028_e28a();
    }
    return;
}

pub fn pass1_1028_af08(param_1: u32) -> u16 {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let paVar3: *mut Struct493;
    let local_bx_72: *mut Struct824;
    let mut u_var4: u16;
    let ppVar5: *mut pass1_struct_1;
    let mut in_stack_0000ffde: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;

    process_struct_1010_20ba(_g_Struct372_1050_0ed0, CONCAT22(in_stack_0000ffde, 2));
    if ((u16_1050_13ae < 1) || (SBORROW2(u16_1050_13ae, 1))) {
        // LAB_1028_af27:
        local_c._2_2_ = 1;
    } else {
        if (u16_1050_13ae == 2 || (u16_1050_13ae - 1) < 1) {
            local_c = 0x10001;
            // goto LAB_1028_af42;
        }
        if (u16_1050_13ae != 4) {}
        // goto LAB_1028_af27;
        local_c._2_2_ = 2;
    }
    local_c = CONCAT22(local_c._2_2_, 3);
    // LAB_1028_af42:
    u_var2 = pass1_fn_1008_612e(local_c._2_2_, local_c);
    u_var4 = (param_1 >> 0x10);
    local_bx_72 = param_1;
    local_bx_72.field_0x114 = u_var2;
    ppVar5 = process_struct_1010_20ba(_g_Struct372_1050_0ed0, CONCAT22(in_stack_0000ffde, 0x37));
    u_var2 = (ppVar5 >> 0x10);
    u_var1 = local_bx_72.field_0x108;
    paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
    post_win_msg_1008_a0e4(
        ppVar5,
        0,
        local_bx_72.field_0x114,
        &paVar3[0x11].field_0xa,
        local_bx_72.field_0x108,
        2,
    );
    ppVar5 = process_struct_1010_20ba(_g_Struct372_1050_0ed0, CONCAT22(in_stack_0000ffde, 0x2b));
    pass1_1010_043a(ppVar5, &paVar3.field_0x4, 0xd);
    return 1;
}

pub fn pass1_1028_a188(param_1: u16, param_2: u16, param_1_00: i32, param_2_00: i32, param_5: u32) {
    let mut u_var1: u32;
    let lVar2: u32;
    let mut u_var3: u32;
    let mut u_var4: u16;
    let mut u_var5: i32;
    let mut u_var6: i32;
    let mut u_var7: u32;
    let lVar8: u32;
    let lVar9: u32;
    let mut u_var10: i32;
    let local_bx_6: *mut Struct816;
    let mut i_var11: i32;
    let mut unaff_si: u16;
    let mut u_var12: u16;
    let ppVar13: *mut pass1_struct_1;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u32;

    u_var12 = (param_5 >> 0x10);
    local_bx_6 = param_5;
    u_var1 = &local_bx_6.field_0x1f6;
    u_var6 = &local_bx_6.field_0x1f8;
    u_var5 = u_var1 + 0x18c;
    u_var4 = (u_var1 >> 0x10);
    u_var7 = u_var5;
    pass1_1030_38f2(u_var1, u_var6, param_2_00);
    u_var3 = 100 / param_1_00;
    u_var10 = u_var3 >> 0xf;
    i_var11 = param_2_00 * 4;
    lVar2 = (u_var7 & 0xffff | u_var6 << 0x10) + (i_var11 + u_var5);
    lVar8 = lVar2 / (u_var3 & 0xffff | u_var10 << 0x10);
    lVar9 = lVar8 * (u_var3 & 0xffff | u_var10 << 0x10);
    local_e = lVar2;
    local_c = (lVar2 >> 0x10);
    u_var6 = lVar9;
    (u_var5 + i_var11) = local_e - u_var6;
    (u_var5 + i_var11 + 2) = (local_c - (lVar9 >> 0x10)) - (local_e < u_var6);
    local_12._2_2_ = (lVar8 >> 0x10);
    local_12._0_2_ = lVar8;
    if ((local_12._2_2_ | local_12) != 0) {
        pass1_1030_375a(u_var1, u_var4, param_2_00, local_12, local_12._2_2_);
        if (local_bx_6.field_0x200 != 0x8000002) {
            ppVar13 = process_struct_1010_20ba(_g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x37));
            post_win_msg_1008_a0e4(
                ppVar13,
                0,
                local_12,
                local_bx_6.field_0x208,
                local_bx_6.field_0x4,
                2,
            );
            ppVar13 = process_struct_1010_20ba(_g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x2b));
            pass1_1010_043a(ppVar13, local_bx_6.field_0x4, 0xd);
        }
    }
    return;
}

pub fn pass1_1028_9a02(param_1: *mut Struct806) -> u8 {
    let mut u_var1: u32;
    let local_AX__1: *mut Struct807;
    let paVar2: *mut Struct493;
    let mut u_var3: i32;
    let pu_var4: *mut u16;
    let mut u_var5: u32;
    let mut in_dx: u16;
    let mut extraout_dx: u16;
    let mut extraout_dx_00: i32;
    let mut extraout_dx_01: i32;
    let local_bx_4: *mut Struct806;
    let mut u_var6: u16;
    let mut unaff_ss: u16;
    let pp_var7: *mut pass1_struct_1;
    let u_var8: u8;
    let u_var9: u8;
    let u_var10: u8;
    let u_var11: u8;
    let mut u_var12: u16;
    let mut u_var13: u16;
    let mut u_var14: u16;
    let mut uVar15: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_1e: u16;
    let mut local_1a: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var6 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    u_var1 = local_bx_4.field_0x108;
    paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
    _local_6 = CONCAT22(in_dx, paVar2);
    u_var5 = paVar2[0x10].field_0x16;
    u_var1 = local_bx_4.field_0x110;
    local_a = u_var5;
    pass1_1030_3694(u_var5, (u_var5 >> 0x10), 0, u_var1, (u_var1 >> 0x10));
    u_var3 = u_var5;
    local_bx_4.field_0x114 = u_var3;
    local_bx_4.field_0x116 = extraout_dx;
    pass1_1030_38b8(local_a, (local_a >> 0x10));
    if ((extraout_dx_00 | u_var3) == 0) {
        local_12 = (_local_6 + 0x200);
        pp_var7 = process_struct_1010_20ba(_g_Struct372_1050_0ed0, CONCAT22(local_38, 0x2b));
        local_14 = (pp_var7 >> 0x10);
        local_16 = pp_var7;
        if (local_12 == 0x8000002) {
            u_var14 = 0x1f;
        } else {
            u_var14 = 0xb;
        }
        pass1_1010_043a(pp_var7, (_local_6 + 4), u_var14);
        if (local_12 == 0x8000001) {
            u_var6 = 2;
        } else {
            u_var6 = 1;
        }
        local_12 = CONCAT22(0x800, u_var6);
        pass1_1038_349e(_local_6, CONCAT22(0x800, u_var6));
        local_1e = 0;
        local_1a = 0;
        pass1_1028_dc52(
            CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, &local_30)),
            (&PTR_LOOP_1050_0000 + 1),
            0,
            0x400,
        );
        while (true) {
            pu_var4 = &local_30;
            pass1_1028_e4ec(CONCAT22(unaff_ss, pu_var4));
            _local_6 = CONCAT22(extraout_dx_01, pu_var4);
            if ((extraout_dx_01 | pu_var4) == 0) {
                break;
            }
            if ((pu_var4 + 0x100) == 0x8000002) {
                local_1a = 1;
            } else {
                local_1e = 1;
            }
        }
        if (local_1e == 0) {
            u_var13 = 0;
            uVar15 = 0x3c;
            u_var10 = 1;
            u_var11 = 0;
            u_var12 = 0;
            u_var6 = 0;
            u_var14 = 0;
            u_var8 = 0;
            u_var9 = 0;
            pp_var7 = process_struct_1010_20ba(_g_Struct372_1050_0ed0, 0x37);
            post_win_msg_1008_a0e4(
                pp_var7,
                CONCAT22(u_var6, CONCAT11(u_var9, u_var8)),
                u_var14,
                CONCAT11(u_var11, u_var10),
                CONCAT22(u_var13, u_var12),
                uVar15,
            );
        }
    }
    return 0x1;
}

pub fn pass1_1028_74e4(param_1: u32) -> u16 {
    pass1_1028_7fb6(param_1);
    pass1_1028_7c4e(param_1);
    pass1_1028_7dfc(param_1);
    post_msg_1028_76da(param_1);
    pass1_1028_767e(param_1);
    pass1_1028_75bc();
    pass1_1028_78b8(param_1);
    return 1;
}

pub fn pass1_1028_6ff6(param_1: u32) {
    let pu_var1: *mut u16;
    let paVar2: *mut Struct493;
    let mut u_var3: u16;
    let mut u_var4: u32;
    let mut extraout_dx: i32;
    let mut extraout_dx_00: i32;
    let mut extraout_dx_01: i32;
    let mut extraout_dx_02: i32;
    let mut u_var5: i32;
    let mut extraout_dx_03: i32;
    let mut extraout_dx_04: i32;
    let mut unaff_ss: u16;
    let ppVar6: *mut pass1_struct_1;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let u_var9: u8;
    let u_var10: u8;
    let u_var11: u8;
    let mut u_var12: u16;
    let mut u_var13: u16;
    let mut u_var14: u16;
    let mut local_48: u16;
    let mut local_46: u16;
    let mut local_44: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_26: u32;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var10 = (unaff_ss >> 8);
    pass1_1028_dc52(
        CONCAT13(u_var10, CONCAT12(unaff_ss, &local_14)),
        (&PTR_LOOP_1050_0000 + 1),
        0,
        0x400,
    );
    local_1a = 1;
    local_1c = 0;
    while {
        while {
            pu_var1 = &local_14;
            pass1_1028_e4ec(CONCAT22(unaff_ss, pu_var1));
            _local_18 = CONCAT22(extraout_dx, pu_var1);
            if ((extraout_dx | pu_var1) == 0) {}
            // goto LAB_1028_7066;
            (pu_var1[0xff] == 0) || ((pu_var1 + 0x100) == 0x8000002)
        } {}
        local_1c = 1;
        u_var4 = (pu_var1 + 0xfb);
        local_26 = u_var4;
        pass1_1030_38b8(u_var4, (u_var4 >> 0x10));
        (extraout_dx_00 < 0) || (extraout_dx_00 < 1 && (u_var4 == 0))
    } {}
    local_1a = 0;
    // LAB_1028_7066:
    local_a = local_6;
    local_c = local_8;
    if (local_4 != 0) {
        local_a = 0;
        local_c = 1;
    }
    local_1e = 0;
    while (true) {
        pu_var1 = &local_14;
        pass1_1028_e4ec(CONCAT22(unaff_ss, pu_var1));
        _local_18 = CONCAT22(extraout_dx_01, pu_var1);
        local_20 = extraout_dx_01 | pu_var1;
        if (local_20 == 0) {
            break;
        }
        if ((pu_var1 + 0x100) == 0x8000001) {
            local_1e = 1;
        }
    }
    if (local_1e == 0) {
        paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 1, 0x400);
        _local_18 = CONCAT22(local_20, paVar2);
        local_20 = local_20 | paVar2;
        if (local_20 != 0) {
            PTR_LOOP_1050_4fe8 = (&PTR_LOOP_1050_0000 + 1);
            u_var3 = 0;
            u_var14 = 1;
            _local_34 = process_struct_1010_20ba(_g_Struct372_1050_0ed0, 0x2b);
            pass1_1010_089e(_local_34, u_var3, u_var14);
            pass1_1010_089e(_local_34, 0, 2);
            pass1_1010_089e(_local_34, 0, 3);
            pass1_1010_089e(_local_34, 0, 4);
            pass1_1010_089e(_local_34, 0, 5);
            pass1_1010_089e(_local_34, 0, 7);
            pass1_1010_089e(_local_34, 0, 8);
            pass1_1010_089e(_local_34, 0, 10);
            local_20 = extraout_dx_02;
        }
    }
    if ((local_1c != 0) && (local_1a != 0)) {
        u_var13 = 0;
        u_var14 = 6;
        u_var9 = 1;
        u_var11 = 0;
        u_var12 = 0;
        u_var8 = 0;
        u_var3 = 0;
        u_var7 = 0;
        _local_34 = process_struct_1010_20ba(_g_Struct372_1050_0ed0, 0x37);
        local_20 = (_local_34 >> 0x10);
        post_win_msg_1008_a0e4(
            _local_34,
            CONCAT22(u_var8, u_var7),
            u_var3,
            CONCAT11(u_var11, u_var9),
            CONCAT22(u_var13, u_var12),
            u_var14,
        );
    }
    local_22 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 1, 0x800);
    u_var5 = local_20 | local_22;
    if (((((u_var5 != 0)
        && (
            u_var3 = pass1_1030_2242(CONCAT22(local_20, local_22), 4),
            u_var3 == 0,
        ))
        && (
            u_var3 = pass1_1030_2242(CONCAT22(local_20, local_22), 0x2a),
            u_var3 == 0,
        ))
        && ((
            u_var3 = pass1_1030_2242(CONCAT22(local_20, local_22), 0x4b),
            u_var3 == 0
                && (
                    u_var3 = pass1_1030_2242(CONCAT22(local_20, local_22), 0x54),
                    u_var3 == 0,
                ),
        )))
        && ((
            u_var3 = pass1_1030_2242(CONCAT22(local_20, local_22), 0x2c),
            u_var3 == 0
                && ((
                    u_var3 = pass1_1030_2242(CONCAT22(local_20, local_22), 0x3c),
                    u_var3 == 0
                        && (
                            u_var3 = pass1_1030_2242(CONCAT22(local_20, local_22), 0x3d),
                            u_var3 == 0,
                        ),
                )),
        )))
    {
        if (local_4 != 0) {
            local_8 = 1;
            local_6 = 0;
        }
        _local_34 = (_local_34 & 0xffff0000);
        local_30 = 0;
        local_c = local_8;
        local_a = local_6;
        while {
            while {
                pu_var1 = &local_14;
                pass1_1028_e4ec(CONCAT22(unaff_ss, pu_var1));
                _local_18 = CONCAT22(extraout_dx_03, pu_var1);
                u_var5 = extraout_dx_03 | pu_var1;
                if (u_var5 == 0) {}
                // goto LAB_1028_72d3;
                (pu_var1 + 0x100) == 0x8000002
            } {}
            u_var3 = (param_1 >> 0x10);
            if ((local_34 == 0)
                && (
                    pass1_1028_740c(param_1, u_var3, 0x22, CONCAT22(extraout_dx_03, pu_var1)),
                    pu_var1 != 0x0,
                ))
            {
                _local_34 = CONCAT22(local_32, 1);
            }
            if ((local_30 == 0)
                && (
                    pass1_1028_740c(param_1, u_var3, 0x24, _local_18),
                    pu_var1 != 0x0,
                ))
            {
                local_30 = 1;
            }
            (local_34 == 0) || (local_30 == 0)
        } {}
        u_var13 = 0;
        u_var14 = 0x14;
        u_var9 = 1;
        u_var11 = 0;
        u_var12 = 0;
        u_var8 = 0;
        u_var3 = 0;
        u_var7 = 0;
        local_26 = process_struct_1010_20ba(_g_Struct372_1050_0ed0, 0x37);
        u_var5 = (local_26 >> 0x10);
        post_win_msg_1008_a0e4(
            local_26,
            CONCAT22(u_var8, u_var7),
            u_var3,
            CONCAT11(u_var11, u_var9),
            CONCAT22(u_var13, u_var12),
            u_var14,
        );
    }
    // LAB_1028_72d3:
    paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 1, 0x400);
    _local_18 = CONCAT22(u_var5, paVar2);
    if ((u_var5 | paVar2) != 0) {
        ppVar6 = process_struct_1010_20ba(_g_Struct372_1050_0ed0, CONCAT22(local_48, 0x3b));
        local_2e = (ppVar6 >> 0x10);
        local_30 = ppVar6;
        pass1_1008_df4a(ppVar6);
        ppVar6 = process_struct_1010_20ba(_g_Struct372_1050_0ed0, CONCAT22(local_48, 0x3c));
        local_2e = (ppVar6 >> 0x10);
        local_30 = ppVar6;
        pass1_1018_34a6(ppVar6);
        pass1_1028_dc52(
            CONCAT13(u_var10, CONCAT12(unaff_ss, &local_46)),
            (&PTR_LOOP_1050_0000 + 1),
            0,
            0x400,
        );
        while (true) {
            pu_var1 = &local_46;
            pass1_1028_e4ec(CONCAT22(unaff_ss, pu_var1));
            _local_34 = CONCAT22(extraout_dx_04, pu_var1);
            if ((extraout_dx_04 | pu_var1) == 0) {
                break;
            }
            if ((pu_var1 + 0x100) != 0x8000002) {
                pass1_1038_3ba0(CONCAT22(extraout_dx_04, pu_var1));
            }
        }
    }
    return;
}

pub fn win_msg_fn_1020_d460(param_1: *mut u8, param_2: u32, param_3: u32, param_4: u32) -> u16 {
    let mut u_var1: u16;
    let ppVar2: *mut pass1_struct_1;
    let mut in_stack_0000ffe8: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    u_var1 = pass1_1028_bc90(param_1, param_2, param_3, param_4);
    if (u_var1 != 0) {
        pass1_1038_af40(_g_Struct112_a, *(_PTR_LOOP_1050_4230 + 0x16), 0x11);
        PTR_LOOP_1050_5b80 = (&PTR_LOOP_1050_0000 + 1);
        process_win_msg_1008_9510(&PTR_LOOP_1050_5b80, &g_alloc_addr_1050_1050);
        ppVar2 =
            process_struct_1010_20ba(_g_Struct372_1050_0ed0, CONCAT22(in_stack_0000ffe8, 0x3a));
        (param_1 + 0x20) = (ppVar2 + 10);
        u_var1 = 1;
    }
    return u_var1;
}

pub fn get_sys_metrics_1020_7c1a(param_1: *mut u16, param_2: u32) {
    let mut u_var1: u16;
    let i_var2: u16;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut local_10: u16;

    u_var4 = (param_2 >> 0x10);
    u_var1 = (param_2 + 8);
    u_var5 = (param_1 >> 0x10);
    i_var3 = param_1;
    unsafe {
        *param_1 = s_1_1050_389a;
    }
    (i_var3 + 2) = &PTR_LOOP_1050_1008;
    unsafe {
        *param_1 = (s_18_2_1050_3aa5 + 3);
    }
    (i_var3 + 2) = &PTR_LOOP_1050_1008;
    (i_var3 + 4) = u_var1;
    unsafe {
        *param_1 = s_0_020_1050_3ab0;
    }
    (i_var3 + 2) = &PTR_LOOP_1050_1008;
    (i_var3 + 6) = param_2;
    (i_var3 + 10) = 0;
    (i_var3 + 0xe) = 0;
    (i_var3 + 0x10) = 0;
    (i_var3 + 0x12) = 0;
    unsafe {
        *param_1 = 0x7f72;
    }
    (i_var3 + 2) = 0x1020;
    (i_var3 + 10) = (param_2 + 0xe4);
    i_var2 = GetSystemMetrics16(4);
    (i_var3 + 0xe) = i_var2;
    i_var2 = GetSystemMetrics16(5);
    (i_var3 + 0x10) = i_var2;
    i_var2 = GetSystemMetrics16(6);
    (i_var3 + 0x12) = i_var2;
    return;
}

pub fn post_win_msg_1020_79fc(param_1: u32, param_2: u16, param_3: u16, param_4: i32) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut i_var3: i32;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut u_var6: u16;

    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    u_var6 = (*(i_var4 + 0xe0) >> 0x10);
    ppc_var2 = ((i_var4 + 0xe0) + 0x24);
    i_var3 = ppc_var2();
    if (i_var3 != param_2) {
        PostMessage16(0, 0, 0x85, (i_var4 + 8));
        u_var1 = (i_var4 + 0xe0);
        ppc_var2 = ((i_var4 + 0xe0) + 0x28);
        ppc_var2(offset, u_var1, (u_var1 >> 0x10), param_2, u_var6);
    }
    return;
}

pub fn call_win_fn_1020_7526(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    win_fn_1020_7270(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn post_msg_1020_4394(param_1: u32, uparam_2: i32) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut u_var3: u16;

    i_var2 = param_1;
    u_var3 = (param_1 >> 0x10);
    if (param_2 == 0x10) {
        if ((i_var2 + 0x34) != 0) {
            PostMessage16(0, 0xf6, 0x111, g_h_window);
            return;
        }
    } else {
        if (param_2 < 0x11) {
            if (param_2 == 0x1) {
                (i_var2 + 0x18) = 0;
                return;
            }
            if (param_2 == 0xb) {
                u_var1 = (i_var2 + 0x2c);
                (u_var1 + 0xe) = (i_var2 + -0xda);
                return;
            }
        }
    }
    return;
}

pub fn cleanup_fn_1020_28fc(param_1: *mut Struct376) {
    param_1.ptr_a_lo = (s__P__P__P__P__P__P__P__P__P__P__P_1050_2e35 + 0x15);
    (param_1 + 2) = 0x1020;
    destroy_menu_func_1020_795c(param_1);
    return;
}

pub fn post_win_msg_1020_291a(param_1: u32) {
    PostMessage16(0, 0, 0x10, (param_1 + 8));
    return;
}

pub fn post_win_msg_1020_1ca4(param_1: u32) -> u16 {
    let pp_var1: fn();
    let mut u_var2: u16;
    let mut u_var3: i32;
    let mut u_var4: u16;
    let pu_var5: *mut u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    u_var4 = (param_1 >> 0x10);
    if ((param_1 + 0x96) == 0) {
        pu_var5 = pass1_1010_4df0((param_1 + 0x8e));
        if (pu_var5 == 0) {
            u_var4 = 0x1000;
            process_struct_1000_179c(0xb4, (pu_var5 >> 0x10));
            u_var3 = (pu_var5 >> 0x10) | pu_var5;
            if (pu_var5 == 0x0) {
                u_var2 = 0;
                u_var3 = 0;
            } else {
                u_var4 = SUB42(&PTR_LOOP_1050_1040, 0);
                u_var2 = mixed_1040_8520(pu_var5, g_h_window, 0x30, 2, 0x57b, 0x62a);
            }
            _local_a = CONCAT22(u_var3, u_var2);
            pp_var1 = (*_local_a + 0x74);
            (**pp_var1)(u_var4, u_var2, u_var3);
            return 0;
        }
        PostMessage16(0, 0xde, 0x111, g_h_window);
    }
    return 1;
}

pub fn DestroyWindow16(handle: HWND16) -> bool {
    todo!();
    false
}

pub fn IsWindow16(handle: HWND16) -> bool {
    todo!();
    false
}

pub fn DeleteObject16(obj: HGDIOBJ16) -> bool {
    todo!();
    false
}

pub fn DestroyMenu16(hmenu: HMENU16) -> bool {
    todo!();
    false
}

pub fn RemoveProp16(hwnd: HWND16, str: &String) -> HANDLE16 {
    todo!();
    false
}

pub fn GetClientRect16(hwnd: HWND16, rect: &mut RECT16) {
    todo!()
}

pub fn InvalidateRect16(hwnd: HWND16, rect: &mut RECT16, erase: bool) {
    todo!()
}

pub fn GetDlgItem16(hwnd_dlg: HWND16, id: i16) -> HWND16 {
    todo!()
}

pub fn EnableWindow16(hwnd: HWND16, enable: bool) -> bool {
    todo!()
}

pub fn ShowWindow16(hwnd: HWND16, cmd: i16) -> bool {
    todo!()
}

pub fn SendMessage16(hwnd: HWND16, msg: u16, wparam: WPARAM16, lparam: LPARAM) -> LRESULT {
    todo!()
}

pub fn FreeProcInstance16(func: fn() -> LRESULT) {
    todo!()
}

pub fn BeginPaint16(hwnd: HWND16, lps: PAINTSTRUCT16) -> HDC16 {
    todo!()
}

// HPEN16 WINAPI CreatePen16( INT16 style, INT16 width, COLORREF color )
pub fn CreatePen16(style: i16, width: i16, color: COLORREF) -> HPEN16 {
    todo!()
}

// HGDIOBJ16 WINAPI SelectObject16( HDC16 hdc, HGDIOBJ16 handle )
pub fn SelectObject16(hdc: HDC16, handle: HGDIOBJ16) -> HGDIOBJ16 {
    todo!()
}

// HBRUSH16 WINAPI CreateSolidBrush16( COLORREF color )
pub fn CreateSolidBrush16(color: COLORREF) -> HBRUSH16 {
    unimplemented!()
}

// HPALETTE16 WINAPI SelectPalette16( HDC16 hdc, HPALETTE16 hpal, BOOL16 bForceBackground )
pub fn SelectPalette16(hdc: HDC16, hpal: HPALETTE16, bForceBackground: bool) -> HPALETTE16 {
    unimplemented!()
}

// BOOL16 WINAPI EndPaint16( HWND16 hwnd, const PAINTSTRUCT16* lps )
pub fn EndPaint16(hwnd: HWND16, lps: PAINTSTRUCT16) -> bool {
    unimplemented!()
}

// BOOL16 WINAPI Polygon16( HDC16 hdc, const POINT16* pt, INT16 count )
pub fn Polygon16(hdc: HDC16, pt: POINT16, count: i16) -> bool {
    unimplemented!()
}

// BOOL16 WINAPI PtInRect16( const RECT16 *rect, POINT16 pt )
pub fn PtInRect16(rect: RECT16, pt: POINT16) -> bool {
    unimplemented!()
}

// HDC16 WINAPI GetDC16( HWND16 hwnd )
pub fn GetDC16(hwnd: HWND16) -> HDC16 {
    unimplemented!()
}

// HDC16 WINAPI CreateDC16( LPCSTR driver, LPCSTR device, LPCSTR output, const DEVMODEA  *initData )
pub fn CreateDC16(driver: &String, device: &String, output: &String, init_data: DEVMODEA) -> HDC16 {
    unimplemented!()
}

// HGDIOBJ16 WINAPI GetStockObject16( INT16 obj )
pub fn GetStockObject16(obj: i16) -> HGDIOBJ16 {
    unimplemented!()
}

// BOOL16 WINAPI Rectangle16( HDC16 hdc, INT16 left, INT16 top,                           INT16 right, INT16 bottom )
pub fn Rectangle16(hdc: HDC16, left: i16, top: i16, right: i16, bottom: i16) -> bool {
    unimplemented!()
}

// BOOL16 WINAPI DeleteDC16( HDC16 hdc )
pub fn DeleteDC16(hdc: HDC16) -> bool {
    unimplemented!()
}

// INT16 WINAPI GetDlgCtrlID16( HWND16 hwnd )
pub fn GetDlgCtrlID16(hwnd: HWND16) -> i16 {
    unimplemented!()
}

// COLORREF WINAPI SetTextColor16( HDC16 hdc, COLORREF color )
pub fn SetTextColor16(hdc: HDC16, color: COLORREF) {
    unimplemented!()
}

// COLORREF WINAPI SetBkColor16( HDC16 hdc, COLORREF color )
pub fn SetBkColor16(hdc: HDC16, color: COLORREF) -> COLORREF {
    unimplemented!()
}

// DWORD WINAPI MoveTo16( HDC16 hdc, INT16 x, INT16 y )
pub fn MoveTo16(hdc: HDC16, x: i16, y: i16) -> u32 {
    unimplemented!()
}

// BOOL16 WINAPI LineTo16( HDC16 hdc, INT16 x, INT16 y )
pub fn LineTo16(hdc: HDC16, x: i16, y: i16) -> bool {
    unimplemented!()
}

// DWORD WINAPI GetCurrentPosition16( HDC16 hdc )
pub fn GetCurrentPosition16(hdc: HDC16) -> u32 {
    unimplemented!()
}

// INT16 WINAPI FrameRect16( HDC16 hdc, const RECT16 *rect16, HBRUSH16 hbrush )
pub fn FrameRect16(hdc: HDC16, rect: &mut RECT16, hbrush: HBRUSH16) -> i16 {
    unimplemented!()
}

// UINT16 WINAPI RealizePalette16( HDC16 hdc )
pub fn RealizePalette16(hdc: HDC16) -> u16 {
    unimplemented!()
}

// HPALETTE16 WINAPI CreatePalette16( const LOGPALETTE* palette )
pub fn CreatePalette16(palette: &LOGPALETTE) -> HPALETTE16 {
    unimplemented!()
}

// BOOL16 WINAPI TextOut16( HDC16 hdc, INT16 x, INT16 y, LPCSTR str, INT16 count )
pub fn TextOut16(hdc: HDC16, x: i16, y: i16, _str: &String, count: i16) -> bool {
    unimplemented!()
}

// BOOL16 WINAPI PostMessage16( HWND16 hwnd, UINT16 msg, WPARAM16 wparam, LPARAM lparam )
pub fn PostMessage16(hwnd: HWND16, msg: u16, wparam: WPARAM16, lparam: LPARAM) -> bool {
    unimplemented!()
}

// INT16 WINAPI FillRect16( HDC16 hdc, const RECT16 *rect, HBRUSH16 hbrush )
pub fn FillRect16(hdc: HDC16, rect: &RECT16, hbrush: HBRUSH16) -> i16 {
    unimplemented!()
}

// INT16 WINAPI DrawText16( HDC16 hdc, LPCSTR str, INT16 count, LPRECT16 rect, UINT16 flags )
pub fn DrawText16(hdc: HDC16, _str: &String, count: i16, rect: RECT16, flags: u16) -> i16 {
    unimplemented!()
}

// INT16 WINAPI ReleaseDC16( HWND16 hwnd, HDC16 hdc )
pub fn ReleaseDC16(hwnd: HWND16, hdc: HDC16) -> i16 {
    unimplemented!()
}

// BOOL16 WINAPI UnrealizeObject16( HGDIOBJ16 obj )
pub fn UnrealizeObject16(obj: HGDIOBJ16) -> bool {
    unimplemented!()
}

// INT16 WINAPI SetMapMode16( HDC16 hdc, INT16 mode )
pub fn SetMapMode16(hdc: HDC16, mode: i16) -> i16 {
    unimplemented!()
}

/*
BOOL16 WINAPI GrayString16( HDC16 hdc, HBRUSH16 hbr, GRAYSTRINGPROC16 gsprc,
                            LPARAM lParam, INT16 cch, INT16 x, INT16 y,
                            INT16 cx, INT16 cy )
*/
pub fn GrayString16(
    hdc: HDC16,
    hbr: HBRUSH16,
    gsprc: GRAYSTRINGPROC16,
    lparam: LPARAM,
    cch: i16,
    x: i16,
    y: i16,
    cx: i16,
    cy: i16,
) -> bool {
    unimplemented!()
}

// INT16 WINAPI GetSystemMetrics16( INT16 index )
pub fn GetSystemMetrics16(index: i16) -> i16 {
    unimplemented!()
}

// HANDLE16 WINAPI GetProp16( HWND16 hwnd, LPCSTR str )
pub fn GetProp16(hwnd: HWND16, str: &String) -> HANDLE16 {
    unimplemented!()
}

// void WINAPI ScreenToClient16( HWND16 hwnd, LPPOINT16 lppnt )
pub fn ScreenToClient16(hwnd: HWND16, lppnt: &POINT16) {
    unimplemented!()
}

// BOOL16 WINAPI IsIconic16(HWND16 hwnd)
pub fn IsIconic16(hwnd: HWND16) -> bool {
    unimplemented!()
}

// HDC16 WINAPI GetWindowDC16( HWND16 hwnd )
pub fn GetWindowDC16(hwnd: HWND16) -> HDC16 {
    unimplemented!()
}

// void WINAPI GetWindowRect16( HWND16 hwnd, LPRECT16 rect )
pub fn GetWindowRect16(hwnd: HWND16, rect: &RECT16) {
    unimplemented!()
}

// LONG WINAPI GetWindowLong16( HWND16 hwnd16, INT16 offset )'
pub fn GetWindowLong16(hwnd16: HWND16, offset: i16) -> libc::c_long {
    unimplemented!()
}

// INT16 WINAPI lstrlen16( LPCSTR str )
pub fn lstrlen16(_str: &String) -> i16 {
    unimplemented!()
}

// DWORD WINAPI GetTextExtent16( HDC16 hdc, LPCSTR str, INT16 count )
pub fn GetTextExtent16(hdc: HDC16, _str: &String, count: i16) -> u32 {
    unimplemented!()
}
