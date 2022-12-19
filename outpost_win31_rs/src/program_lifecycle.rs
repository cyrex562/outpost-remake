//
// Created by cyrex on 2022-05-24.
//

// #include "types.h"
// #include "structs_2.h"
// #include "func_ptrs.h"
// #include "entry.h"
// #include "utils.h"
// #include "globals.h"
// #include "sys_api.h"
// #include "block_1000.h"

use std::os::raw::c_char;
use crate::block_1000::block_1000_2000::{exit_op_1000_2950, fn_ptr_op_1000_2594, init_1000_23be, pass1_1000_25a8, pass1_1000_27d6, pass1_1000_2913, pass1_1000_29dc, poss_str_op_1000_28dc};
use crate::block_1000::block_1000_5000::{dos3_call_1000_23ea, ret_op_1000_55ac};
use crate::globals::{DAT_1050_5f82, DAT_1050_5f87, HINSTANCE16_1050_5f4c, PTR_LOOP_1050_0000, PTR_LOOP_1050_5f48, PTR_LOOP_1050_5f4a, PTR_LOOP_1050_5f4e, PTR_LOOP_1050_5f50, PTR_LOOP_1050_5f7e, PTR_LOOP_1050_5f84, PTR_LOOP_1050_5fb8, PTR_LOOP_1050_5fc2, PTR_LOOP_1050_5fc4, PTR_LOOP_1050_5fd2, PTR_LOOP_1050_5fd4, PTR_LOOP_1050_63fe, REG_DI, REG_SI, u8_1050_5fc9, WIN_VERSION_1050_5f80};
use crate::winbase::{FatalAppExit16, FatalExit, GetModuleFileName16, GetVersion16, InitApp16, InitTask16, LockSegment16, WaitEvent16};
use crate::utils::CONCAT22;
use std::ptr;
use std::ptr::null_mut;
use crate::app_context::AppContext;
use crate::block_1000::block_1000_2000;
use crate::dos_interrupt;
use crate::dos_interrupt::{dos3_op_1000_256b, swi};
use crate::mem_container::MemContainer;
use crate::structs::struct_822::Struct822;
use crate::structs::struct_825::Struct825;

pub unsafe fn entry(
    ctx: &mut AppContext,
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: i16,
    param_4: *mut u8,
    mut param_5: u16,
    param_6: u16,
) -> *mut c_char {
    let mut win_version: u32 = 0u32;
    let mut var10: u32 = 0u32;
    let mut pstruct_825_var13: *mut Struct825 = ptr::null_mut();

    let mut u32_var11 = CONCAT22(param_6, PTR_LOOP_1050_5f84 as u16);
    loop {
        InitTask16(null_mut());
        PTR_LOOP_1050_5f84 = u32_var11;
        if param_3 != 0x0 {
            PTR_LOOP_1050_5f7e = 0x1050;
            let mut var9 = param_5 <  0xff00;
            param_5 = param_5 + 0x100;
            if var9 {
                PTR_LOOP_1050_5f50 = 0x1050;
                PTR_LOOP_1050_5f48 = param_5;
                PTR_LOOP_1050_5f4a = ctx.SI_REG;
                HINSTANCE16_1050_5f4c = ctx.DI_REG;
                PTR_LOOP_1050_5f4e = param_4;
                LockSegment16(0xffff);
                PTR_LOOP_1050_5f84 = u32_var11;
                win_version = GetVersion16();
                PTR_LOOP_1050_5f84 = u32_var11;
                WIN_VERSION_1050_5f80 = win_version;
                // TODO: find value of AH to figure out which syscall was requested
                let mut func_ptr_3 = swi(ctx, 0x21);
                let mut u32_var12 = u32_var11;
                u32_var11 = func_ptr_3::CODE8();
                PTR_LOOP_1050_5f84 = u32_var12;
                DAT_1050_5f82 = u32_var11 as u16;
                DAT_1050_5f87 = 0;
                WaitEvent16(0x0);
                PTR_LOOP_1050_5f84 = u32_var11;
                param_3 = InitApp16(HINSTANCE16_1050_5f4c);
                PTR_LOOP_1050_5f84 = u32_var11;
                if param_3 != 0x0 {
                    break;
                }
            }
        }
        param_3 = ((param_3 >> 0x8) << 0x8) | 0xff;
        entry_1000_24db(ctx, param_3 as u16);
        PTR_LOOP_1050_5f84 = u32_var11;
    }
    dos3_call_1000_23ea(param_4, 0x1050, 0x0, 0x1050);
    PTR_LOOP_1050_5f84 = u32_var11;
    entry_1000_262c(u32_var11, 0x238d, 0x1538);
    PTR_LOOP_1050_5f84 = u32_var11;
    pass1_1000_27d6(u32_var11 as u16);
    win_version = ret_op_1000_55ac();
    let var4 = win_version as u16;
    init_1000_23be(param_5, (var10 >> 0x10) as u16);
    exit_op_1000_24cd(ctx, var4);
    // TODO
    // paVar13 =  CONCAT22(uVar4, 0x15);
    pass1_1000_25a8();
    pass1_1000_2913(0x15);
    let mut string_var4 = poss_str_op_1000_28dc(pstruct_825_var13);
    if string_var4.is_null() == false {
        let mut var5 = 0x9;
        if string_var4[0] == 'M' {
            var5 = 0xf;
        }
        string_var4 = string_var4 + var5;
        let mut varb = 0x22;
        let mut string_var8 = string_var4;
        loop {
            if varb == 0x0 {
                break;
            }
            varb += -0x1;
            let var1 = string_var8;
            string_var8 = string_var8 + 1;
            if !(var1.field0_0x0 != '\r') {
                break;
            }
        }
        (string_var8 -1) = '\0';
    }
    FatalAppExit16(0x0, string_var4);
    FatalExit();
    let mut string_var7 = PTR_LOOP_1050_63fe;
    loop {
        let mut string_var1 = string_var7;
        string_var7 = string_var7 + 1;
        let mut string_var5 = string_var7;
        string_var5 = string_var1 + 1;
        // TODO: string_var1 == param_2 where param_2 is a u16 possibly an address
        if string_var5.is_null() {
            return string_var5;
        }
        let mut var6 = -0x1;
        loop {
            if var6 == 0x0 {
                break;
            }
            var6 += -0x1;
            let var2 = string_var7;
            string_var7 = (string_var7 + 1);
            if *var2 == 0 {
                break;
            }
        }
    }
}

// WARNING: Removing unreachable block (ram,0x10002513)
// WARNING: Removing unreachable block (ram,0x10002557)
pub unsafe fn entry_1000_24db(ctx: &mut AppContext, mut param_1: u16) {
    //        1000:254c b4 4c           MOV        AH,0x4c
    ctx.AH_REG = 0x4c;
    let mut sys_call_ptr: *mut code;
    let mut unaff_bp = 0u16;
    let i_var2 = unaff_bp + 1;
    u8_1050_5fc9 = 0;
    block_1000_2000::fn_ptr_op_1000_2594();
    block_1000_2000::fn_ptr_op_1000_2594();
    dos_interrupt::dos3_op_1000_256b();
    let sys_call_ptr = swi(ctx, 0x21);
    sys_call_ptr::CODE(i_var2);
    return;
}

// WARNING (jumptable): Unable to track spacebase fully for stack
// WARNING (jumptable): Heritage AFTER dead removal. Example location: r0x10505fc2 : 0x1000270c
// WARNING: Unable to track spacebase fully for stack
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
// WARNING: Restarted to delay deadcode elimination for space: ram
pub unsafe fn entry_1000_262c(param_1: u16, mut param_2: u16, mut param_3: u16) {


    let mut var3: i16;
    let mut var4: u16;
    let mut var5: u16;
    let mut var6: u16;
    let mut var7: u16;
    let mut var8: u16;
    let mut var10: u16;
    let mut var11: *mut *mut c_char;
    let mut var12: *mut c_char;
    let mut var13: *mut c_char;
    let mut var14: u16;
    let mut var15: u16;
    let mut var16: *mut u8;

    PTR_LOOP_1050_5fd2 = param_2;
    PTR_LOOP_1050_5fd4 = param_3;
    param_3 = 0x263d;
    param_2 = exit_op_1000_2950(ctx, 0x8, param_1, 0x104);
    param_3 = param_1;
    PTR_LOOP_1050_5fc2 = param_2;
    PTR_LOOP_1050_5fc4 = param_1;
    CONCAT22(param_1, param_2);

    let mut out_module_fname: Vec<u8> = Vec::with_capacity(512);
    let mut result_str_len = GetModuleFileName16(0x104, out_module_fname.as_mut_ptr() as *mut c_char, HINSTANCE16_1050_5f4c);
    param_2[result_str_len] = '\0';
    let mut i_var4 = 0x1;
    PTR_LOOP_1050_5fb8 = (PTR_LOOP_1050_0000 + 1);
// TODO:
// LAB_1000_266c:
    let mut var1 = 0u16;
    let mut var2 = 0u16;
    let mut var7 = PTR_LOOP_1050_5f7e;
    loop {
        loop {
            let mut var17 = var7;
            var7 = var7 + 1;
            var2 = var17;
            if var2 != ' ' {
                break;
            }
        }
        if var2 != '\t' {
            break;
        }
    }
    if (var2 != '\r') && (var2 != '\0') {
        PTR_LOOP_1050_5fb8 = PTR_LOOP_1050_5fb8 + 1;
        loop {
            var7 -= 1;
// TODO:
// LAB_1000_267f:
            var1 = var7;
            var7 += 1;
            var2 = *var1;
            if (var2 == ' ') || (var2 == '\t') {
                // TODO:
                // goto LAB_1000_266c;
            }
            if (var2 == '\r') || (var2 == '\0') {
                break;
            }
            if var2 == '\"' {
// LAB_1000_26b8:
                loop {
                    loop {
                        loop {
                            var1 = var7;
                            var7 = var7 + 1;
                            var2 = *var1;
                            if (var2 == '\r') || (var2 == '\0') {
                                // TODO:
                                // goto LAB_1000_26e8;
                            }
                            if var2 == '\"' {
                                // TODO:
                                // goto LAB_1000_267f;
                            }
                            if var2 == '\\' {
                                break;
                            }
                            i_var4 += 0x1;
                        }
                        var7 = 0;
                        loop {
                            var13 = var7;
                            var7 += 0x1;
                            var7 = var13 + 1;
                            var2 = *var13;
                            if var2 != '\\' {
                                break;
                            }
                        }
                        if var2 == '\"' {
                            break;
                        }
                        i_var4 += var7;
                        var7 = var13;
                    }
                    i_var4 = i_var4 + (var7 >> 1) + ((var7 & 1) != 0);
                    if (var7 & 1) == 0 {
                        break;
                    }
                }
                // TODO:
                // goto LAB_1000_267f;
            }
            if var2 != '\\' {
                i_var4 += 0x1;
                // TODO:
                // goto LAB_1000_267f;
            }
            var6 = 0;
            loop {
                var6 += 0x1;
                var1 = var7;
                var7 = var7 + 1;
                var2 = *var1;
                if var2 != '\\' {
                    break;
                }
            }
            if var2 == '\"' {
                i_var4 = i_var4 + (var6 >> 1) + ((var6 & 1) != 0);
                if (var6 & 1) == 0 {
                    // TODO:
                    // goto LAB_1000_26b8;
                }
                // TODO:
                // goto LAB_1000_267f;
            }
            i_var4 += var6;
        }
    }
// TODO:
// LAB_1000_26e8:
    param_3 = 0x1050u16;
    var3 = -((PTR_LOOP_1050_5fb8 + (PTR_LOOP_1050_5fb8 + 1) * 0x4 + i_var4 + 1) & 0xfffe);
    PTR_LOOP_1050_5fba = &stack0x0004 + var3;
    PTR_LOOP_1050_5fbc = 0x1050u16;
    var13 = stack0x0004 + (PTR_LOOP_1050_5fb8 + 1) * 0x4 + var3;
    param_3 + var3 = 0x1050u16;
    var16 = PTR_LOOP_1050_5fc4;
    var14 = (param_3 + var3);
    (&stack0x0004 + var3) = PTR_LOOP_1050_5fc2;
    (&stack0x0006 + var3) = var16;
    var11 = (&stack0x0008 + var3);
    param_3 + var3 = stack0x0004 + var3;
    param_2 + var3 = 0x1538;
    stack0xfffe + var3 = 0x271f;
    var4 = pass1_1000_29dc(0x1050);
    var15 = &PTR_LOOP_1050_5f7e;
    var7 = 0x0081;
// TODO:
// LAB_1000_272e:
    loop {
        loop {
            var1 = var7;
            var7 = var7 + 1;
            var2 = *var1;
            if var2 != ' ' {
                break;
            }
        }
        if var2 != '\t' {
            break;
        }
    }
    if (var2 == '\r') || (var2 == '\0') {
//  TODO:
//  LAB_1000_27c1:
        (param_3 + var3) = 0x1538;
        (param_2 + var3) = 0x27c5;
        var5 = block_1000_2000::pass1_1000_29dc(0x1050);
        *var11 = null_mut();
        var11[0x1] = null_mut();
        // WARNING: Could not recover jumptable at 0x100027d2. Too many branches
        // WARNING: Treating indirect jump as call
        PTR_LOOP_1050_5fd2();
        _PTR_LOOP_1050_5fc2 = CONCAT22(PTR_LOOP_1050_5fc4, PTR_LOOP_1050_5fc2);
        return;
    }
    *var11 = var13;
    var11[0x1] = 0x1050;
    var11 = var11 + 2;
    loop {
        var7 = var7 -0x1;
//  TODO:
//  LAB_1000_274f:
        var1 = var7;
        var7 = var7 + 1;
        var2 = *var1;
        if (var2 == ' ') || (var2 == '\t') {
            var1 = var13;
            var13 = var13 + 1;
            *var1 = '\0';
            // TODO:
            // goto LAB_1000_272e;
        }
        if (var2 == '\r') || (var2 == '\0') {
            //
            //            LAB_1000_27be:
            *var13 = '\0';
            // TODO:
            // goto LAB_1000_27c1;
        }
        var12 = var7;
        if var2 == '\"' {
// TODO:
// LAB_1000_278b:
            loop {
                var7 = var12 + 1;
                var2 = *var12;
                if (var2 == '\r') || (var2 == '\0') {
                    // TODO:
                    // goto LAB_1000_27be;
                }
                if var2 == '\"' {
                    break;
                }
                if var2 == '\\' {
                    var10 = 0;
                    loop {
                        var12 = var7;
                        var10 += 0x1;
                        var7 = var12 + 1;
                        var2 = *var12;
                        if var2 != '\\' {
                            break;
                        }
                    }
                    if var2 == '\"' {
                        // for (uVar11 = uVar10 >> 0x1; uVar11 != 0; uVar11 -= 1)
                        for uVar11 in uVar10 >> 1..0 {
                            var1 = var13;
                            var13 = var13 + 1;
                            *var1 = '\\';
                        }
                        if (var10 & 1) == 0 {
                            break;
                        }
                        var1 = var13;
                        var13 = var13 + 1;
                        *var1 = '\"';
                        var12 = var7;
                    } else {
                        // for (; uVar10 != 0; uVar10 -= 1) {
                        while uVar10 != 0 {
                            var1 = var13;
                            var13 = var13 + 1;
                            *var1 = '\\';
                            uVar10 -= 1;
                        }
                    }
                } else {
                    var1 = var13;
                    var13 = var13 + 1;
                    *var1 = var2;
                    var12 = var7;
                }
            }
            // TODO:
            // goto LAB_1000_274f;
        }
        if var2 != '\\' {
            var1 = var13;
            var13 = var13 + 1;
            *var1 = var2;
            // TODO:
            // goto LAB_1000_274f;
        }
        var8 = 0;
        loop {
            var8 += 0x1;
            var1 = var7;
            var7 = var7 + 1;
            var2 = *var1;
            if var2 != '\\' {
                break;
            }
        }
        if var2 == '\"' {
            // for (uVar9 = uVar8 >> 0x1; uVar9 != 0; uVar9 -= 1)
            for uVar9 in uVar8 >> 0x1..0 {
                var1 = var13;
                var13 = var13 + 1;
                *var1 = '\\';
            }
            var12 = var7;
            if (var8 & 1) == 0 {
                // TODO:
                // goto LAB_1000_278b;
            }
            var1 = var13;
            var13 = var13 + 1;
            *var1 = '\"';
            // TODO: goto LAB_1000_274f;
        }
        // for (; uVar8 != 0; uVar8 -= 1)
        while uVar8 != 0 {
            var1 = var13;
            var13 = var13 + 1;
            *var1 = '\\';
            uVar8 -= 1;
        }
    }
}

// WARNING: Removing unreachable block (ram,0x10002557)
pub unsafe fn exit_op_1000_24cd(ctx: &mut AppContext, mut param_1: u16) {
    let mut var2: i16;
    let mut var3: u16;
    let mut var4: u16;
    let mut var5: u16;
    let mut var6: u16;
    let mut var7: u16;

    u8_1050_5fc9 = u8::try_from('\0').unwrap();
    fn_ptr_op_1000_2594();
    fn_ptr_op_1000_2594();
    ret_op_1000_55ac();
    fn_ptr_op_1000_2594();
    fn_ptr_op_1000_2594();
    dos3_op_1000_256b(ctx);
    // terminate with return code
    ctx.AH_REG = 0x4c;
    let result = swi(ctx, 0x21);
    result.call(ctx);
}
