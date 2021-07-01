use crate::app_context::AppContext;
use crate::exit::{exit_1000_25cc, exit_1000_2950, return_1000_39e1};
use crate::func_ptr_funcs::{call_fn_ptr_1000_256b, call_fn_ptr_1000_2594};
use crate::other_funcs::exported_stub_1000_29dc;
use crate::pass::pass_funcs::{pass1_fn_1000_29af, pass1_fn_1000_29b5, pass1_fn_1000_3bac, pass1_fn_1000_462e};
use crate::string_ops::misc::process_string_1000_55b1;
use crate::structs::prog_structs_25::Struct152;
use crate::typedefs::SEGPTR;
use crate::util::{CARRY2, CONCAT11, POPCOUNT, SUB42};
use crate::winapi;
use crate::winapi::GetDOSEnviornment16;

pub unsafe fn get_dos_env_1000_27d6(ctx: &mut AppContext) {
    let pi_var1: i32;
    let pc_var2: String;
    let pi_var3: i32;
    let mut c_var4: char;
    let mut string_var5: String;
    let mut i32_var6: i32;
    let mut i_var7: i32;
    let mut pi_var8: i32;
    let mut pi_var9: i32;
    let mut pc_var10: String;
    let mut pi_var11: i32;
    let mut b_var12: bool;
    let mut u_var13: String;
    let mut u_var14: String;
    let mut pc_var15: String;
    let mut u_var16 = &ctx.g_alloc_addr_1050_1050;
    let mut dos_env = GetDOSEnviornment16();
    // dos_env = (dos_env >> 0x10);
    // if dos_env != 0 {
    //     dos_env = 0;
    // }
    let mut i_var7 = 0;
    let mut pc_var10 = String::new();
    let mut i32_var6 = -1;
    if dos_env != 0 {
        c_var4 = '\0';
        while c_var4 != '\0' {
            while {
                if i32_var6 == 0 {
                    break;
                }
                i32_var6 = i32_var6 - 1;
                pc_var2 = pc_var10;
                pc_var10 = pc_var10[1..].clone();
                pc_var2[0] != '\0'
            } {}
            i_var7 = i_var7 + 1;
            pc_var2 = pc_var10;
            pc_var10 = pc_var10[1..].clone();
            c_var4 = pc_var2[0];
        }
    }
    u_var13 = exit_1000_2950(ctx, u_var16);
    u_var14 = exit_1000_2950(ctx, None);
    //// c_var15 = (u_var13  >> 0x10);
    pc_var10 = u_var13;
    //// Var16 = (u_var14  >> 0x10);
    ctx.PTR_LOOP_1050_5fbe = u_var14;
    pi_var8 = 0x0;
    loop {
        ctx.PTR_LOOP_1050_5fc0 = u_var14.clone();
        string_var5 = u_var14.clone();
        if i_var7 == 0 {
            string_var5[0] = 0x0;
            string_var5[1] = 0x0;
            return;
        }
        b_var12 = pi_var8 == ctx.s__C_FILE_INFO__1050_5f5c[0];
        if b_var12 {
            pi_var11 = ctx.s__C_FILE_INFO__1050_5f5c;
            i32_var6 = 6;
            pi_var9 = pi_var8;
            while {
                if i32_var6 == 0 {
                    break;
                }
                i32_var6 = i32_var6 - 1;
                pi_var3 = pi_var11;
                pi_var11 = pi_var11 + 1;
                pi_var1 = pi_var9;
                pi_var9 = pi_var9 + 1;
                b_var12 = *pi_var1 == *pi_var3;
                b_var12
            } {}
            if !b_var12 {}
            // goto LAB_1000_2867;
        } else {
            // LAB_1000_2867:
            string_var5[0] = pc_var10;
            string_var5[1] = pc_var15.clone();
            // u_var14 = CONCAT22(ctx.PTR_LOOP_1050_5fc0, ppcVar5 + 2);
        }
        while {
            ctx.PTR_LOOP_1050_5fc0 = u_var14.clone();
            pi_var1 = pi_var8;
            pi_var8 = (pi_var8 + 1);
            c_var4 = *pi_var1;
            pc_var2 = pc_var10.clone();
            pc_var10 = pc_var10[1..].clone();
            pc_var2[0] = c_var4;
            // u_var14 = (u_var14 & 0xffff) | ctx.PTR_LOOP_1050_5fc0;
            c_var4 != '\0'
        } {}
        i_var7 = i_var7 - 1;
    }
}

pub unsafe fn get_dos_env_1000_27da() {
    let pi_var1: i32;
    let pc_var2: String;
    let pi_var3: i32;
    let mut c_var4: char;
    let mut i_var5: i32;
    let mut i32_var6: i32;
    let mut u_var7: u16;
    char * *ppc_var8;
    let mut i_var9: i32;
    let pi_var10: i32;
    let pi_var11: i32;
    let pc_var12: String;
    let pi_var13: i32;
    let mut b_var14: bool;
    let s_var15: SEGPTR;
    let mut u_var16: String;
    let mut pc_var17: String = String::new();

    s_var15 = GetDOSEnviornment16();
    //// 32_var6 = (s_var15  >> 0x10);
    // if s_var15 != 0 {
    //     i32_var6 = 0;
    // }
    i_var9 = 0;
    pc_var12 = String::new();
    i_var5 = -1;
    if i32_var6 != 0 {
        c_var4 = *0x0;
        while c_var4 != '\0' {
            while {
                if i_var5 == 0 {
                    break;
                }
                i_var5 = i_var5 + -1;
                pc_var2 = pc_var12;
                pc_var12 = pc_var12[1..].clone();
                pc_var2[0] != '\0'
            } {}
            i_var9 = i_var9 + 1;
            pc_var2 = pc_var12;
            pc_var12 = pc_var12[1..].clone();
            unsafe {
                c_var4 = pc_var2[0];
            }
        }
    }
    u_var16 = exit_1000_2950(ctx, None);
    //// c_var17 = (u_var16  >> 0x10);
    pc_var12 = u_var16;
    u_var16 = exit_1000_2950(ctx, None);
    //// _var7 = (u_var16  >> 0x10);
    ppc_var8 = u_var16;
    // 0x5fbe = ppc_var8;
    // ctx.PTR_LOOP_1050_5fc0 = u_var7;
    pi_var10 = 0x0;
    loop {
        if i_var9 == 0 {
            ppc_var8[0] = 0x0;
            ppc_var8[1] = 0x0;
            return;
        }
        unsafe {
            b_var14 = pi_var10[0] == ctx.s__C_FILE_INFO__1050_5f5c;
        }
        if b_var14 {
            pi_var13 = ctx.s__C_FILE_INFO__1050_5f5c;
            i_var5 = 6;
            pi_var11 = pi_var10;
            while {
                if i_var5 == 0 {
                    break;
                }
                i_var5 = i_var5 + -1;
                pi_var3 = pi_var13;
                pi_var13 = pi_var13 + 1;
                pi_var1 = pi_var11;
                pi_var11 = pi_var11 + 1;
                unsafe {
                    b_var14 = *pi_var1 == *pi_var3;
                }
                b_var14
            } {}
            if !b_var14 {}
            // goto LAB_1000_2867;
        } else {
            // LAB_1000_2867:
            ppc_var8 = pc_var12[0];
            ppc_var8[1] = u_var16;
            ppc_var8 = ppc_var8 + 2;
        }
        while {
            pi_var1 = pi_var10;
            pi_var10 = (pi_var10 + 1);

                c_var4 = *pi_var1;

            pc_var2 = pc_var12;
            pc_var12 = pc_var12[1..].clone();

                pc_var2[0] = c_var4;
            c_var4 != '\0'
        } {}
        i_var9 = i_var9 + -1;
    }
}

pub unsafe fn dos3_call_1000_2bb6(ctx: &mut AppContext, param_1: &String, param_2: &mut Struct152) -> u32 {
    let mut pu8_var1: &Vec<u8>;
    let mut struct_var2: Struct152;
    let mut u8_var3: u8;
    let mut i_var4: &String;
    let mut i_var5: &String;
    let mut str_var6: &String;
    let mut u16_var7: &Vec<u8>;
    let mut i32_var8: i32;

    // struct_var2 = param_2;
    i32_var8 = unaff_bp + 1;
    let mut u16_var7 = &ctx.g_alloc_addr_1050_1050;
    let mut u8_var3 = &param_2.field_0xa;
    if ((u8_var3 & 0x82) != 0) && ((u8_var3 & 0x40) == 0) {
        param_2.field_0x4 = 0;
        if (u8_var3 & 1) != 0 {
            if (u8_var3 & 0x10) == 0 {}
            // goto LAB_1000_2c37;
            param_2.field_0x0 = param_2.field_0x6.clone();
            u8_var3 = u8_var3 & 0xfe;
        }
        param_2.field_0xa = u8_var3 & 0xef | 2;
        str_var6 = &param_2.field_0xb;
        if ((u8_var3 & 8) == 0)
            && ((u8_var3 & 4) != 0
                || ((&param_2.field_0xf0 & 1) == 0
                    && (ctx.PTR_LOOP_1050_61ec != 0x0
                        && ((param_2 == 0x621c || (param_2 == 0x6228))
                            && ((*(str_var6 + 0x5f90) & 0x40) != 0))
            //             || (
            // process_struct_1000_2ce8(param_2, ctx.dx_reg),
            // (struct_var2.field_0xa & 8) == 0,
            //             )
        )))
        {
            i_var4 = dos3_call_1000_39f2(
                &mut str_var6,
                &param_1,
                None
            );
            i_var5 = 1;
        } else {
            i_var4 = &param_2.field_0x6;
            // i_var5 = &param_2.field_0x0 - i_var4;
            param_2.field_0x0 = i_var4[1..].clone();
            param_2.field_0x4 = param_2.field_0xf2  - 1;
            if i_var5 == 0 {
                // i_var4 = 0;
                if (*(str_var6 + 0x5f90) & 0x20) != 0 {
                    dos3_call_1000_3636(&str_var6, 0, 0, 2);
                    // i_var4 = 0;
                    i_var5 = i_var4;
                }
            } else {
                i_var4 = dos3_call_1000_39f2(&mut str_var6, &struct_var2.field_0x6, Some(&mut struct_var2.field_0x8), i_var5);
            }
           struct_var2.field_0x6 = param_1.clone();
        }
        if i_var4 == i_var5 {
            return param_1 & 0xff;
        }
    }
    // LAB_1000_2c37:
    pu8_var1 = &struct_var2.field_0xa;
    pu8_var1[0] = pu8_var1[0] | 0x20;
    return 0xffff;
}

pub unsafe fn dos3_call_1000_35fe(ctx: &mut AppContext, param_1: i32) -> u16 {
    let pc_var1: code;
    let mut u_var2: u16;
    let mut u8_var3: bool;

    if param_1 < u16_1050_5f8a {
        u8_var3 = false;
        pc_var1 = winapi::swi(0x21);
        unsafe {
            u_var2 = (*pc_var1)(ctx.bp_reg + 1);
        }
        if !u8_var3 {
            *(param_1 + 0x5f90) = 0;
        }
    } else {
        u_var2 = 0x900;
        u8_var3 = true;
    }
    if u8_var3 {
        pass1_fn_1000_29b5(u_var2);
        return 0xffff;
    }
    return 0;
}

pub unsafe fn dos3_call_1000_3636(param_1: &String, param_2: i32, uparam_3: i32, param_4: i32) {
    let pu8_var1: Vec<u8>;
    let pc_var2: code;
    let mut u_var3: u16;
    let mut unaff_bp: i32;
    let mut u8_var4: bool;
    let mut u_var5: u32;
    let mut local_8: u16;
    let mut local_6: u16;

    if (((param_1 < u16_1050_5f8a) || (ctx.PTR_LOOP_1050_61ec == 0x0)) || (2 < param_1)) {
        if ((ctx.PTR_LOOP_1050_6064 == 0x0) || ((param_3 & 0x8000) == 0)) {}
        // goto LAB_1000_36e3;
        if (param_4 == 0) {}
        // goto LAB_1000_369b;
        u8_var4 = false;
        pc_var2 = winapi::swi(0x21);
        unsafe {
            u_var5 = (*pc_var2)();
        }
        u_var3 = u_var5;
        if (u8_var4) {}
        // goto LAB_1000_299d;
        if ((param_4 & 2) == 0) {
            if (-1 < ((u_var5 >> 0x10) + param_3 + CARRY2(u_var3, param_2))) {
                // LAB_1000_36e3:
                u8_var4 = false;
                pc_var2 = winapi::swi(0x21);
                unsafe {
                    u_var3 = (*pc_var2)();
                }
                if (!u8_var4) {
                    pu8_var1 = (param_1 + 0x5f90);
                    u8_var4 = false;
                    unsafe {
                        *pu8_var1 = *pu8_var1 & 0xfd;
                    }
                }
                // goto LAB_1000_299d;
            }
        } else {
            pc_var2 = winapi::swi(0x21);
            unsafe {
                u_var5 = (*pc_var2)(unaff_bp + 1);
            }
            if (-1 < ((u_var5 >> 0x10) + param_3 + CARRY2(u_var5, param_2))) {}
            // goto LAB_1000_36e3;
            pc_var2 = winapi::swi(0x21);
            unsafe {
                (*pc_var2)();
            }
        }
        // LAB_1000_369b:
        u_var3 = s_471_bmp_1050_1600;
    } else {
        u_var3 = 0x900;
    }
    u8_var4 = true;
    // LAB_1000_299d:
    if (u8_var4) {
        pass1_fn_1000_29b5(u_var3);
    }
    return;
}

pub unsafe fn dos3_call_1000_370a(
    param_1: u16,
    param_2: u16,
    param_3: i32,
    param_4: u8,
    param_5: i32,
) -> u16 {
    let pc_var1: code;
    let mut u_var2: i32;
    let mut u_var3: u16;
    let mut i_var4: i32;
    let mut b_var5: u8;
    let mut u_var6: i32;

    let mut u_var7: u16;
    let mut unaff_bp: i32;
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
    pc_var1 = winapi::swi(0x21);
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
            pc_var1 = winapi::swi(0x21);
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
                pc_var1 = winapi::swi(0x21);
                unsafe {
                    (*pc_var1)();
                }
                bVar9 = false;
                pc_var1 = winapi::swi(0x21);
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
                    pc_var1 = winapi::swi(0x21);
                    unsafe {
                        u_var3 = (*pc_var1)();
                    }
                    u_var8 = unaff_bp + 1;
                    if (bVar9) {}
                    // goto LAB_1000_299d;
                }
            }
            // LAB_1000_3973:
            if ((_local_8 & 0x40) == 0) {
                pc_var1 = winapi::swi(0x21);
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
            pc_var1 = winapi::swi(0x21);
            unsafe {
                (*pc_var1)();
            }
            u_var3 = 0x1800;
        }
    } else {
        if ((u_var6 & 0x500) != 0x500) {
            _local_8 = CONCAT11(1, b_var5);
            pc_var1 = winapi::swi(0x21);
            unsafe {
                (*pc_var1)();
            }
            if ((ctx.dx_reg & 0x80) != 0) {
                _local_8 = _local_8 | 0x40;
            }
            u_var7 = u_var3;
            u_var8 = _local_6;
            if ((_local_8 & 0x40) == 0) {
                if ((param_2 & 0x200) == 0) {
                    if (((_local_8 & 0x80) != 0) && ((param_2 & 2) != 0)) {
                        pc_var1 = winapi::swi(0x21);
                        unsafe {
                            (*pc_var1)();
                        }
                        pc_var1 = winapi::swi(0x21);
                        unsafe {
                            i_var4 = (*pc_var1)();
                        }
                        if ((i_var4 != 0) && (local_5 = (u_var2 >> 8), local_5 == '\x1a')) {
                            pc_var1 = winapi::swi(0x21);
                            unsafe {
                                (*pc_var1)(unaff_bp + 1);
                            }
                            pc_var1 = winapi::swi(0x21);
                            unsafe {
                                (*pc_var1)();
                            }
                        }
                        u_var6 = 0;
                        pc_var1 = winapi::swi(0x21);
                        unsafe {
                            (*pc_var1)();
                        }
                        u_var7 = u_var3;
                        u_var8 = in_stack_0000fff2;
                    }
                } else {
                    if ((param_2 & 3) == 0) {
                        unsafe {
                            pc_var1 = winapi::swi(0x21);
                            (*pc_var1)();
                            pc_var1 = winapi::swi(0x21);
                            (*pc_var1)();
                        }
                        // goto LAB_1000_38e3;
                    }
                    u_var6 = 0;
                    pc_var1 = winapi::swi(0x21);
                    unsafe {
                        (*pc_var1)();
                    }
                    u_var7 = u_var3;
                }
            }
            // goto LAB_1000_3973;
        }
        pc_var1 = winapi::swi(0x21);
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

pub unsafe fn dos3_call_1000_39f2(param_1: &mut String, param_2: &String, param_3: Option<&mut u16>) -> u16 {
    let pc_var1: String;
    let mut u_var2: i32;
    let pcVar3: code;
    let mut u_var4: u16;
    let mut cVar5: u8;
    let mut u_var6: u16;
    let pcVar7: String;
    let mut u_var8: i32;
    let pc_var9: String;
    let mut i_var10: i32;
    let pu_var11: u16;
    let mut unaff_bp: i32;
    let mut unaff_si: u16;
    let pc_var12: String;
    let pc_var13: String;
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

    iStack2 = unaff_bp + 1;
    uStack4 = SUB42(&ctx.g_alloc_addr_1050_1050, 0);
    local_e = u16_1050_5f8a;
    pcVar7 = u16_1050_5f8a;
    if ((ctx.PTR_LOOP_1050_61ec != 0x0)
        && (
            pcVar7 = PTR_s_ed_in_Op_Op_1050_0028_1050_5f8e,
            param_1 < (&dos_alloc_addr_1050_0002 + 1),
        ))
    {
        *param_1 = u16_1050_5f8a;
    }
    if pcVar7 <= param_1 {
        uVar15 = true;
        u_var6 = 0x900;
        // goto LAB_1000_299d;
    }
    pcVar7 = param_1;
    if (param_1[0x5f90] & 0x20) != 0 {
        uVar15 = false;
        pcVar3 = winapi::swi(0x21);
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
            local_c = &ctx.g_alloc_addr_1050_1050;
            uVar15 = pcVar7 < local_e;
            if (uVar15) {
                uVar15 = 0;
                pcVar3 = winapi::swi(0x21);
                unsafe {
                    u_var20 = (*pcVar3)();
                }
            } else {
                local_e = s_2_3_1050_3b71;
                u_var20 = process_string_1000_55b1(ctx);
            }
            u_var6 = u_var20;
            local_a = param_2;
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
        local_a = &ctx.g_alloc_addr_1050_1050;
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
            uStack16 = param_2;
            u_var8 = pass1_fn_1000_3bac();
            u_var4 = uStack16;
            if (u_var8 < 0xa9) {
                uStack18 = unaff_cs;
                uStack18 = bad_1000_25f2();
                if (pc_var13 == pc_var9) {
                    return local_e;
                }
                //TODO
                //bVar16 = param_1 < local_e;
                pc_var12 = param_1 + -local_e;
                cVar19 = pc_var12 < 0;
                cVar18 = pc_var12 == 0x0;
                cVar5 = (POPCOUNT(pc_var12 & 0xff) & 1) == 0;
                if (bVar16) {
                    bVar16 = 0;
                    cVar19 = '\0';
                    cVar18 = 0x1;
                    cVar5 = 0x1;
                    pcVar3 = winapi::swi(0x21);
                    unsafe {
                        u_var8 = (*pcVar3)(&ctx.g_alloc_addr_1050_1050, u_var6, pcVar7);
                    }
                } else {
                    u_var8 = process_string_1000_55b1(ctx);
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
                            cVar5 = dos3_call_1000_3ad9(0);
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
                        cVar5 = dos3_call_1000_3ad9(0);
                    }
                    pc_var1 = pcVar7;
                    pcVar7 = pcVar7 + 1;
                    unsafe {
                        *pc_var1 = cVar5;
                    }
                    *param_3 = *param_3 - 1;
                    *param_3 != 0
                } {}
                (&uStack18 + i_var10) = (ctx.s_0_020_1050_3ab0 + 1);
                dos3_call_1000_3ad9(0);
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

pub unsafe fn dos3_call_1000_3ad9(param_1: u16) -> u16 {
    let pu_var1: u32;
    let pi_var2: i32;
    let mut i_var3: i32;
    let mut u_var4: i32;
    let pcVar5: code;
    let mut u_var6: u16;
    let mut u_var7: i32;
    let mut in_cx: i32;
    let mut u_var8: i32;
    let mut unaff_bp: i32;
    let mut unaff_DI: i32;
    let mut unaff_ss: u16;
    let mut bVar9: u8;
    let mut bVar10: bool;
    let mut cVar11: u8;
    let mut in_af: u8;
    let mut cVar12: u8;
    let mut cVar13: u8;

    if (unaff_DI == ctx.dx_reg) {
        return param_1;
    }
    u_var8 = (unaff_bp + 6);
    pu_var1 = (unaff_bp + -0xc);
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
        pcVar5 = winapi::swi(0x21);
        unsafe {
            u_var7 = (*pcVar5)(&ctx.g_alloc_addr_1050_1050);
        }
    } else {
        u_var7 = process_string_1000_55b1(ctx);
    }
    if (!bVar9) {
        pi_var2 = (unaff_bp + -4);
        unsafe {
            *pi_var2 = *pi_var2 + u_var7;
        }
        bVar9 = in_cx < u_var7;
        u_var4 = in_cx - u_var7;
        cVar13 = u_var4 < 0;
        cVar12 = u_var4 == 0;
        cVar11 = (POPCOUNT(u_var4 & 0xff) & 1) == 0;
        if (bVar9 || cVar12) {
            return param_1;
        }
    }
    u_var4 = (cVar13 << 7 | cVar12 << 6 | in_af << 4 | cVar11 << 2 | 2 | bVar9) << 8;
    u_var6 = u_var7 & 0xff | u_var4;
    if ((unaff_bp + -4) == 0) {
        bVar10 = (u_var4 & 0x100) != 0;
        if (bVar10) {
            u_var6 = CONCAT11(9, (u_var7 & 0xff));
        } else {
            if (((*(u_var8 + 0x5f90) & 0x40) == 0) || (**(unaff_bp + 8) != '\x1a')) {
                bVar10 = true;
                u_var6 = 0x1c00;
            } else {
                bVar10 = false;
            }
        }
    } else {
        u_var8 = (unaff_bp + -4);
        pu_var1 = (unaff_bp + -6);
        unsafe {
            bVar10 = u_var8 < *pu_var1;
            u_var6 = u_var8 - *pu_var1;
        }
    }
    i_var3 = (unaff_bp + -10);
    if bVar10 {
        (i_var3 + 2) = s_fem102_wav_1050_29a2;
        // *(i_var3 + 2)
        pass1_fn_1000_29b5(u_var6, );
        u_var6 = 0xffff;
    }
    return u_var6;
}

pub unsafe fn dos3_call_1000_42de(param_1: u32, param_2: &mut u16, param_3: &mut u16) {
    let mut u_var1: u16;
    let mut u_var2: u16;
    let pcVar3: code;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut unaff_bp: i32;
    let mut i32_var6: i32;
    let mut unaff_si: u16;
    let mut i_var7: i32;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut bVar11: bool;
    let mut u_var12: u32;
    let mut u_var13: u16;

    i32_var6 = unaff_bp + 1;
    u_var13 = SUB42(&ctx.g_alloc_addr_1050_1050, 0);
  // u_var8 = (param_1  >> 0x10);
    i_var7 = param_1;
    u_var5 = (i_var7 + 2);
    u_var4 = (i_var7 + 4);
    u_var1 = (i_var7 + 8);
    u_var8 = (i_var7 + 10);
  // u_var9 = (param_3  >> 0x10);
    unsafe {
        u_var10 = *param_3;
    }
    u_var2 = (param_3 + 6);
    bVar11 = false;
    pcVar3 = winapi::swi(0x21);
    unsafe {
        u_var12 = (*pcVar3)();
    }
    unsafe {
        *param_3 = u_var10;
    }
    (param_3 + 6) = u_var2;
  // u_var10 = (param_2  >> 0x10);
    i_var7 = param_2;
    unsafe {
        *param_2 = u_var12;
    }
    (i_var7 + 2) = u_var5;
    (i_var7 + 4) = u_var4;
    (i_var7 + 6) = (u_var12 >> 0x10);
    (i_var7 + 8) = u_var1;
    (i_var7 + 10) = u_var8;
    if bVar11 {
        pass1_fn_1000_29af(ctx.si_reg, u_var13, i32_var6);
    }
    (i_var7 + 0xc) = bVar11;
    return;
}

pub unsafe fn dos3_call_1000_435c(ctx: &mut AppContext, param_1: Option<&mut u16>) {
    let pc_var1: code;
    let mut u_var2: u16;
    let mut in_cx: i32;
    let mut u_var3: i32;


    let mut u_var4: i32;
    let mut u_var5: i32;
    let mut cVar6: u8;

    pc_var1 = winapi::swi(0x21);
    unsafe {
        (*pc_var1)(&ctx.g_alloc_addr_1050_1050);
    }
    pc_var1 = winapi::swi(0x21);
    u_var3 = in_cx;
    u_var5 = ctx.dx_reg;
    unsafe {
        (*pc_var1)();
    }
    cVar6 = u_var3;
    pc_var1 = winapi::swi(0x21);
    unsafe {
        (*pc_var1)(u_var3 >> 8);
    }
    u_var4 = ctx.dx_reg;
    if ((u_var5 != ctx.dx_reg) && (u_var4 = ctx.dx_reg, cVar6 == '\x17')) {
        u_var3 = in_cx;
        u_var4 = u_var5;
    }
    u_var2 = pass1_fn_1000_462e(u_var3 - 0x7bc, u_var4 >> 8, u_var4 & 0xff);
    if (param_1 != 0) {
        (param_1 + 2) = u_var4;
        unsafe {
            *param_1 = u_var2;
        }
    }
    return;
}

pub unsafe fn dos3_call_1000_4f2e() -> u16 {
    let pc_var1: code;
    let mut u_var2: u16;
    let mut unaff_bp: i32;
    let mut u8_var3: bool;

    u8_var3 = false;
    pc_var1 = winapi::swi(0x21);
    unsafe {
        u_var2 = (*pc_var1)(&ctx.g_alloc_addr_1050_1050, unaff_bp + 1);
    }
    if (u8_var3) {
        pass1_fn_1000_29b5(u_var2);
        return 0xffff;
    }
    return 0;
}

pub fn dos3_call_1000_4f94() -> i32 {
    let pc_var1: code;
    let mut b_var2: u8;
    let mut unaff_bp: i32;

    pc_var1 = winapi::swi(0x21);
    unsafe {
        b_var2 = (*pc_var1)(unaff_bp + 1);
    }
    return b_var2 + 1;
}

pub fn dos3_call_1000_4fbe(param_1: u8) -> u16 {
    let pc_var1: code;
    let mut c_var2: u8;
    let mut u_var3: u16;
    let mut unaff_bp: i32;

    pc_var1 = winapi::swi(0x21);
    unsafe {
        (*pc_var1)(unaff_bp + 1);
    }
    pc_var1 = winapi::swi(0x21);
    unsafe {
        c_var2 = (*pc_var1)();
    }
    u_var3 = 0xffff;
    if ((c_var2 + 0x1) == param_1) {
        u_var3 = 0;
    }
    return u_var3;
}

pub unsafe fn dos3_call_1000_514e() -> u16 {
    let pc_var1: code;
    let mut u_var2: u16;
    let mut unaff_bp: i32;
    let mut u8_var3: bool;

    u8_var3 = false;
    pc_var1 = winapi::swi(0x21);
    unsafe {
        u_var2 = (*pc_var1)(&ctx.g_alloc_addr_1050_1050, unaff_bp + 1);
    }
    if (u8_var3) {
        pass1_fn_1000_29b5(u_var2);
        return 0xffff;
    }
    return 0;
}

pub unsafe fn dos3_call_1000_5174() -> u32 {
    let pc_var1: code;
    let mut u_var2: u16;
    let mut unaff_bp: i32;
    let mut u8_var3: bool;

    u8_var3 = false;
    pc_var1 = winapi::swi(0x21);
    unsafe {
        u_var2 = (*pc_var1)(unaff_bp + 1);
    }
    if (!u8_var3) {
        return 0;
    }
    pass1_fn_1000_29b5(u_var2);
    return u_var2 & 0xff;
}

pub unsafe fn dos3_call_1000_51aa(param_1: &mut String, uparam_2_00: &String, param_2: u16) -> u32 {
    let pc_var1: code;

    pc_var1 = winapi::swi(0x21);
    unsafe {
        (*pc_var1)(&ctx.g_alloc_addr_1050_1050);
        pc_var1 = winapi::swi(0x21);
        (*pc_var1)();
        pc_var1 = winapi::swi(0x21);
        (*pc_var1)();
        pc_var1 = winapi::swi(0x21);
        (*pc_var1)();
    }
    if (param_2_00 & 0x100) == 0 {
        return 0;
    }
    pass1_fn_1000_29b5(param_2);
    return param_2 & 0xff;
}

pub unsafe fn dos3_call_1000_23ea(ctx: &mut AppContext, a: i16, b: u16) {
    let pu8_var1: Vec<u8>;
    let pu8_var2: Vec<u8>;
    let mut u8_var3: u8;
    let mut u8_var4: u8;
    let pu8_var5: Vec<u8>;
    let mut i32_var6: i32;
    let pfn_var7: fn();
    let pfn_var8: fn();
    let mut u_var9: u16;
    let mut i_var10: i32;
    let str_142: String;
    let pu8_var11: Vec<u8>;
    let pu8_var12: Vec<u8>;
    let mut u16_segment: u16;
    let mut b_var14: bool;
    let mut u16_var15: u16;
    let mut pfn_var16: u32;

    // DOS API
    pfn_var8 = winapi::swi(0x21);
    pfn_var8(ctx.bp_reg + 1);
    // DOS API
    pfn_var8 = winapi::swi(0x21);
    g_u16_ptr_1050_5f6a = b;
    ctx.PTR_LOOP_1050_5f6c = ctx.es_reg;
    pfn_var8();
    u16_segment = 0x1000;
    u16_var15 = u16_segment;
    u_var9 = exported_stub_1000_29dc();
    if &ctx.g_fn_ptr_1050_6202 != 0 {
        b_var14 = false;
        pfn_var7 = &ctx.g_fn_ptr_1050_6200;
        u16_var15 = u16_segment;
        pfn_var7();
        if b_var14 {
            exit_1000_25cc(ctx);
            return;
        }
        pfn_var16 = 0x6200;
        u16_var15 = u16_segment;
        pfn_var16();
    }
    i32_var6 = (ctx.s_New_failed_in_Op__Op_1050_0020 + 0xc);
    if i32_var6 != 0 {
        pu8_var11 = 0x0;
        while {
            b_var14 = *pu8_var11 == 0;
            if b_var14 {
                break;
            }
            i_var10 = 0xd;
            str_142 = ctx.s__C_FILE_INFO__1050_5f5c;
            while {
                if i_var10 == 0 {
                    break;
                }
                i_var10 = i_var10 + -1;
                pu8_var5 = pu8_var11;
                pu8_var11 = pu8_var11 + 1;
                pu8_var1 = str_142;
                str_142 = (str_142 + 1);
                b_var14 = *pu8_var1 == *pu8_var5;
                b_var14
            } {}
            if (b_var14) {
                pu8_var12 = 0x5f90;
                // goto LAB_1000_2495;
            }
            i_var10 = 0x7fff;
            b_var14 = true;
            while {
                if (i_var10 == 0) {
                    break;
                }
                i_var10 = i_var10 + -1;
                pu8_var1 = pu8_var11;
                pu8_var11 = pu8_var11 + 1;
                b_var14 = *pu8_var1 == 0;
                !b_var14
            } {}
            b_var14
        } {}
    }
    // LAB_1000_24a9:
    u16_var15 = ctx.s_266_bmp_1050_24ae + 4;
    call_fn_ptr_1000_2594();
    u16_var15 = ctx.s_264_bmp_1050_24b6 + 5;
    call_fn_ptr_1000_2594();
    u16_var15 = 0x24c4;
    call_fn_ptr_1000_2594();
    return;
    // LAB_1000_2495:
    pu8_var2 = pu8_var11 + 1;
    unsafe {
        u8_var3 = *pu8_var11;
    }
    if (u8_var3 < 0x41) {}
    // goto LAB_1000_24a9;
    pu8_var11 = pu8_var11 + 2;
    unsafe {
        u8_var4 = *pu8_var2;
    }
    if (u8_var4 < 0x41) {}
    // goto LAB_1000_24a9;
    pu8_var1 = pu8_var12;
    pu8_var12 = pu8_var12 + 1;
    unsafe {
        *pu8_var1 = u8_var4 + 0xbf | (u8_var3 + 0xbf) * '\x10';
    }
    // goto LAB_1000_2495;
}

pub fn dos_api_call_1000_24ff(ctx: &mut AppContext, dos_api_val: u16) {
    let pc_var1: code;
    let mut c_var2: u8;

    ctx.PTR_LOOP_1050_5fc9 = 1;
    c_var2 = 0x1;
    call_fn_ptr_1000_2594();
    call_fn_ptr_1000_2594();
    call_fn_ptr_1000_256b();
    if c_var2 == '\0' {
        pc_var1 = winapi::swi(0x21);
        unsafe {
            (*pc_var1)();
        }
    }
    return;
}
