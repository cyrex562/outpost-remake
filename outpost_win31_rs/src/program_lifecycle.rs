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
use crate::block_1000::block_1000_2000::{exit_op_1000_2950, fn_ptr_op_1000_2594, init_1000_23be, pass1_1000_25a8, pass1_1000_2913, pass1_1000_29dc, poss_str_op_1000_28dc};
use crate::block_1000::block_1000_5000::{dos3_call_1000_23ea, ret_op_1000_55ac};
use crate::globals::{DAT_1050_5f82, DAT_1050_5f87, HINSTANCE16_1050_5f4c, PTR_LOOP_1050_0000, PTR_LOOP_1050_5f48, PTR_LOOP_1050_5f4a, PTR_LOOP_1050_5f4e, PTR_LOOP_1050_5f50, PTR_LOOP_1050_5f7e, PTR_LOOP_1050_5f84, PTR_LOOP_1050_5fb8, PTR_LOOP_1050_5fc2, PTR_LOOP_1050_5fc4, PTR_LOOP_1050_5fd2, PTR_LOOP_1050_5fd4, PTR_LOOP_1050_63fe, REG_DI, REG_SI, u8_1050_5fc9, WIN_VERSION_1050_5f80};
use crate::winbase::{FatalAppExit16, FatalExit, GetModuleFileName16, GetVersion16, InitApp16, InitTask16, LockSegment16, MakeProcInstance16, WaitEvent16};
use crate::utils::CONCAT22;
use std::ptr;
use std::ptr::null_mut;
use crate::app_context::AppContext;
use crate::block_1000::block_1000_2000;
use crate::block_1010::block_1010_0000;
use crate::block_1020::block_1020_1000::{destroy_win_1020_1dea, destroy_win_1020_1e1e, pass1_1020_1da8, pass1_1020_1eea};
use crate::block_1040::block_1040_a000;
use crate::dos_interrupt;
use crate::dos_interrupt::{dos3_op_1000_256b, swi};
use crate::mem_container::MemContainer;
use crate::structs::struct_822::Struct822;
use crate::structs::struct_825::Struct825;
use crate::sys_ops::pass1_1000_27d6;

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


pub unsafe fn mixed_win_sys_op_1008_016e(param_1: *mut astruct_823) {
    let mut puVar1: *mut u16;
    let mut uVar6: u16;
    let mut iVar3: i16;
    let mut uVar5: u16;
    let mut uVar9: u16;
    let mut uVar11: u16;
    let mut uVar8: u32;
    let mut DVar10: u16;
    let mut puVar4: *mut u8;
    let mut puVar14: u16;
    let mut uVar13: u16;
    let mut puVar12: *mut u8;
    let mut puVar13: *mut u8;
    let mut uVar7: u16;
    let mut in_EDX: u32;
    let mut struct_1: *mut astruct_832;
    let mut unaff_DI: i16;
    let mut uVar10: u16;
    let mut uVar12: u16;
    let mut DVar16: u32;
    let mut puVar17: *mut u32;
    let mut pSVar18: *mut StructD;
    let mut in_stack_0000fe46: u16;
    let mut local_13e: [u8; 0xac] = [0; 0xac];
    let mut local_92: [u8; 0x80] = [0; 0x80];
    let mut uStack18: u16;
    let mut uStack16: u16;
    let mut puStack14: *mut u32;
    let mut uStack10: u16;
    let mut uStack8: u16;
    let mut uStack6: u16;
    let mut uStack4: u16;
    let mut uVar1: u32;
    let mut puVar2: *mut u16;
    let mut uVar4: *mut astruct_20;
    let mut uVar2: *mut astruct_20;
    let mut uVar3: *mut astruct_20;
    let mut paVar14: *mut Struct57;
    let mut paVar15: *mut Struct57;
    let mut fn_ptr: *mut *mut code;

    uVar9 = (in_EDX >> 0x10);
    DVar16 = GetVersion16();
    DVar10 = (DVar16 >> 0x10);
    paVar14 = CONCAT22(uVar9, DVar10);
    uStack6 = (DVar16 & 0xffff);
    uVar6 = DVar16 & 0xff;
    uStack10 = ((DVar16 & 0xffff) >> 0x8);
    uStack8 = uVar6;
    if ((uVar6 < 0x3) || (uVar6 == 0x3 && (uStack10 < 0xa))) {
        // 0x97
        uVar12 = 0x1000;
        mem_op_1000_179c(0xb4, paVar14);
        uStack16 = paVar14;
        puVar4 = (uStack16 | uVar6);
        paVar14 = (paVar14 & 0xffff0000);
        paVar15 = (paVar14 | ZEXT24(puVar4));
        uStack18 = uVar6;
        if (puVar4.is_null()) {
            iVar3 = 0;
        } else {
            uVar12 = &PTR_LOOP_1050_1040;
            iVar3 = string_1040_8520(paVar15, CONCAT22(uStack16, uVar6), 0x0, 0x20010, 0x5dd05de);
            paVar14 = paVar15;
        }
        puStack14 = CONCAT22(paVar14, iVar3);
        fn_ptr = (*puStack14 + 0x74);
        (**fn_ptr)(uVar12, iVar3, paVar14);
        fn_ptr_op_1000_24cd(1);
    }
    debug_print_1008_6048(paVar14, s_version__d__d_1050_0012);
    if ((uStack8 == 0x3) && (0xb < uStack10)) {
        PTR_LOOP_1050_0010 = (&PTR_LOOP_1050_0000 + 1);
    }
    LoadString16(
        0x80,
        CONCAT22(0x1050, local_92),
        0x578,
        HINSTANCE16_1050_038c,
    );
    uVar5 = dos3_call_1000_51aa(local_92, &DAT_1050_1050, 1);
    if (uVar5 != 0) {
        LoadString16(
            0x80,
            CONCAT13(0x10, CONCAT12(0x50, local_13e)),
            0x57b,
            HINSTANCE16_1050_038c,
        );
        LoadString16(
            0x80,
            CONCAT13(0x10, CONCAT12(0x50, &stack0xfe42)),
            0x62e,
            HINSTANCE16_1050_038c,
        );
        uVar5 = MessageBox16(
            0x10,
            CONCAT13(0x10, CONCAT12(0x50, local_13e)),
            CONCAT22(0x1050, &stack0xfe42),
            0x0,
        );
        fn_ptr_op_1000_24cd(1);
    }
    mem_op_1000_179c(0x4, paVar14);
    uStack16 = paVar14;
    paVar14 = (paVar14 & 0xffff0000);
    if ((uStack16 | uVar5) == 0) {
        uVar9 = 0;
        uStack18 = uVar5;
    } else {
        uStack18 = uVar5;
        puVar17 = pass1_1008_5394(CONCAT22(uStack16, uVar5));
        paVar14 = (paVar14 & 0xffff0000 | puVar17 >> 0x10);
        uVar9 = SUB42(puVar17, 0x0);
    }
    uVar10 = (param_1 >> 0x10);
    struct_1 = param_1;
    struct_1.field5_0x8 = uVar9;
    (struct_1.field5_0x8 + 0x2) = paVar14;
    uVar8 = struct_1.field5_0x8;
    puVar1 = struct_1.field5_0x8;
    _PTR_LOOP_1050_0298 = uVar8;
    *puVar1 = 0x70;
    // 0x1538
    (puVar1 + 0x2) = s_tile2_bmp_1050_1538;
    mem_op_1000_179c(0x126, paVar14);
    uVar11 = uVar8;
    uStack16 = paVar14;
    paVar15 = (paVar14 & 0xffff0000 | (uStack16 | uVar11));
    uStack18 = uVar11;
    if ((uStack16 | uVar11) != 0) {
        pSVar18 = pass1_1010_2024((uVar8 & 0xffff | paVar14 << 0x10));
        paVar15 = (paVar15 & 0xffff0000 | pSVar18 >> 0x10);
        uVar11 = pSVar18;
    }
    if (_u16_1050_0ed0 == 0) {
        debug_print_1008_6048(paVar15, s_New_failed_in_Op__Op_1050_0020);
        fn_ptr_op_1000_24cd(1);
    }
    mem_op_1000_179c(0xe8c, paVar15);
    uStack16 = paVar15;
    puVar12 = (uStack16 | uVar11);
    paVar14 = (paVar15 & 0xffff0000 | ZEXT24(puVar12));
    uStack18 = uVar11;
    if (puVar12.is_null() == false) {
        pass1_1010_7e40(puVar12, CONCAT22(uStack16, uVar11));
    }
    if (_u16_1050_14cc == 0) {
        debug_print_1008_6048(paVar14, s_New_failed_in_Op__Op__ResLibr_1050_0035);
        fn_ptr_op_1000_24cd(1);
    }
    mem_op_1000_179c(0xb0, paVar14);
    uStack16 = paVar14;
    paVar14 = (paVar14 & 0xffff0000 | (uStack16 | uVar11));
    uStack18 = uVar11;
    if ((uStack16 | uVar11) != 0) {
        pSVar18 = pass1_1038_aeca(CONCAT22(uStack16, uVar11));
        paVar14 = (paVar14 & 0xffff0000 | pSVar18 >> 0x10);
        uVar11 = pSVar18;
    }
    if (_PTR_LOOP_1050_5b7c == 0) {
        debug_print_1008_6048(paVar14, s_New_failed_in_Op__Op__DialogCtr_1050_0053);
        fn_ptr_op_1000_24cd(1);
    }
    mem_op_1000_179c(0xa, paVar14);
    uStack16 = paVar14;
    puVar13 = (uStack16 | uVar11);
    paVar14 = (paVar14 & 0xffff0000 | ZEXT24(puVar13));
    uStack18 = uVar11;
    if (puVar13.is_null() == false) {
        make_proc_inst_1038_cf6c(puVar13, CONCAT22(uStack16, uVar11));
    }
    if (_u16_1050_5bc8 == 0) {
        debug_print_1008_6048(paVar14, s_New_failed_in_Op__Op__DialogHand_1050_0073);
        fn_ptr_op_1000_24cd(1);
    }
    mem_op_1000_179c(0x14, paVar14);
    uStack16 = paVar14;
    paVar14 = (paVar14 & 0xffff0000 | (uStack16 | uVar11));
    uStack18 = uVar11;
    if ((uStack16 | uVar11) != 0) {
        pass1_1008_5bdc(CONCAT22(uStack16, uVar11));
    }
    if (_u16_1050_02a0 == 0) {
        debug_print_1008_6048(paVar14, s_New_failed_in_Op__Op__Simulator_1050_0097);
        fn_ptr_op_1000_24cd(1);
    }
    mem_op_1000_179c(0xfc, paVar14);
    uStack16 = paVar14;
    uVar7 = uStack16 | uVar11;
    uStack18 = uVar11;
    if (uVar7 == 0) {
        uVar11 = 0;
        uVar7 = 0;
    } else {
        set_struct_op_1008_0536(CONCAT22(uStack16, uVar11), in_stack_0000fe46);
    }
    struct_1.field4_0x4 = uVar11;
    (&struct_1.field4_0x4 + 0x2) = uVar7;
    if (struct_1.field4_0x4.is_null()) {
        debug_print_1008_6048(uVar7, s_New_failed_in_Op__Op_1050_00b7);
        fn_ptr_op_1000_24cd(1);
    }
    win_ui_reg_class_1008_96d2(struct_1.field4_0x4);
    fn_ptr = (struct_1.field4_0x4 + 0x8);
    (**fn_ptr)(0x1000);
    uVar4 = struct_1.field4_0x4;
    HWND16_1050_0396 = (uVar4 + 0x8);
    uVar2 = struct_1.field4_0x4;
    fn_ptr = (struct_1.field4_0x4 + 0xc);
    (**fn_ptr)(0x1000, uVar2, (uVar2 >> 0x10), 0x3);
    uVar3 = struct_1.field4_0x4;
    UpdateWindow16((uVar3 + 0x8));
    return;
}


pub unsafe fn mixed_sys_op_1000_40af(
    mut param_1: u16,
    mut param_2: i16,
    mut param_3: u16,
) -> *mut i16 {
    let mut ppa_var1: *mut *mut struct_824;
    let mut pc_var2: *mut c_char;
    let mut pu_var4: *mut u16;
    let mut pc_var5: *mut c_char;
    let mut i_var6: i16;
    let mut ppa_var7: *mut *mut struct_824;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let mut i_var8: i16;
    let mut hglobal_7: HGLOBAL16;
    // pub unsafe fn *SVar8;
    let mut ppa_var8: *mut *mut struct_824;
    let mut unaff_si: i16;
    let mut pi_var9: *mut i16;
    let mut pc_var10: *mut c_char;
    let mut hglobal_di: *mut astruct_824;
    let mut pu_var11: *mut u16;
    let mut unaff_cs: u8;
    let mut unaff_ss: u16;
    let mut b_var12: bool;
    // pub unsafe fn *pvVar13;
    let mut pa_var14: *mut Struct825;
    let mut pu_var3: *mut u16;
    let mut u_var13: u8;
    let mut u_var14: u8;
    let mut i_var12: i16;
    let mut temp_5fa27366cb: *mut astruct_824;

    loop {
        u_var7 = (param_1 * param_3);
        u_var8 = param_2 * param_3 + (param_1 * param_3 >> 0x10);
        if ((u_var8 | u_var7) != 0) {
            pi_var9 = null_mut();
            if ((u_var8 < 0x3) && (u_var8 < 0x2 || (u_var7 == 0))) {
                if (u_var8 == 0) {
                    u_var7 = u_var7 + 0xfff & 0xf000;
                    if (u_var7 == 0) {
                        u_var8 = 0x1;
                    }
                } else if ((param_3 - 0x1 & param_3) != 0) {
                    pi_var9 = ((u_var8 << 0x10) % param_3);
                    b_var12 = CARRY2(u_var7, pi_var9);
                    u_var7 += pi_var9;
                    if (b_var12) {
                        // TODO: goto LAB_1000_41aa;
                    }
                    u_var8 = 0x1;
                }
            } else if ((param_3 - 0x1 & param_3) != 0) {
                // TODO: goto LAB_1000_41aa;
            }
            hglobal_7 = GLobalAlloc16(CONCAT13((u_var8 >> 0x8), CONCAT12(u_var8, u_var7)), 0x20);
            if ((hglobal_7 != 0) && ((u_var7 & 1) != 0)) {
                pvVar13 = WIN16_GlobalLock16(hglobal_7);
                SVar8 = pvVar13;
                if ((SVar8 != 0) || (pvVar13.is_null())) {
                    pa_var14 = CONCAT22(hglobal_7, 0x12);
                    u_var13 = '\x12';
                    u_var14 = '\0';
                    pass1_1000_25a8();
                    pass1_1000_2913(CONCAT11(u_var14, u_var13));
                    pc_var5 = poss_str_op_1000_28dc(pa_var14);
                    if (pc_var5.is_null()) {
                        // TODO: goto LAB_1000_28cb;
                    }
                    i_var6 = 0x9;
                    if (*pc_var5 == 'M') {
                        i_var6 = 0xf;
                    }
                    pc_var5 = pc_var5 + i_var6;
                    i_var6 = 0x22;
                    pc_var10 = pc_var5;
                    break;
                }
                hglobal_7 = block_1000_4000::pass1_1000_422a((pvVar13 >> 0x10), hglobal_7);
                if (hglobal_7 == 0) {
                    GlobalUnlock16(u_var8);
                    GlobalFree16(hglobal_di);
                    hglobal_7 = 0;
                }
            }
            unaff_cs = 0x38;
            if (hglobal_7 != 0) {
                pu_var11 = null_mut();
                // for (; unaff_si != 0; unaff_si += -1)
                while unaff_si != 0 {
                    // for (iVar6 = -0x8000; i_var6 != 0; i_var6 += -1)
                    for ivar6 in -0x8000..0 {
                        pu_var3 = pu_var11;
                        pu_var11 = pu_var11 + 1;
                        *pu_var3 = 0;
                    }
                    hglobal_7 += 0x100;
                    unaff_si -= 1;
                }
                if (hglobal_di.is_null() == false) {
                    // for (; hglobal_di.is_null() == false; hglobal_di = hglobal_di -1)
                    while hglobal_di.is_null() == false {
                        pu_var4 = pu_var11;
                        pu_var11 = (pu_var11 + 1);
                        *pu_var4 = 0;
                        hglobal_di -= 1;
                    }
                }
                return pi_var9;
            }
        } //
          //        LAB_1000_41aa:
        if ((u16_1050_618e | PTR_LOOP_1050_618c) == 0) {
            return NULL;
        }
        i_var8 = (PTR_LOOP_1050_618c)(unaff_cs, param_3, param_1, param_2);
        if (i_var8 == 0) {
            return NULL;
        }
    }
    loop {
        i_var6 += -0x1;
        pc_var2 = pc_var10;
        pc_var10 = pc_var10 + 1;
        if (*pc_var2 == '\r') {
            break;
        }
        if (i_var6 == 0) {
            break;
        }
    }
    pc_var10[-0x1] = '\0'; //
                           //    LAB_1000_28cb:
    FatalAppExit16(CONCAT13(0x10, CONCAT12(0x50, pc_var5)), 0x0);
    FatalExit();
    ppa_var8 = &PTR_LOOP_1050_63fe;
    loop {
        ppa_var1 = ppa_var8;
        ppa_var8 = ppa_var8 + 1;
        temp_5fa27366cb = *ppa_var1;
        ppa_var7 = ppa_var8;
        if ((temp_5fa27366cb == hglobal_di)
            || (ppa_var7 = (temp_5fa27366cb + 1), ppa_var7.is_null()))
        {
            return ppa_var7;
        }
        i_var6 = -0x1;
        loop {
            if (i_var6 == 0) {
                break;
            }
            i_var6 += -0x1;
            ppa_var1 = ppa_var8;
            ppa_var8 = (ppa_var8 + 1);
            if ppa_var1.is_null() {
                break;
            }
        }
    }
}


pub unsafe fn exit_op_1000_2950(ctx: &mut AppContext, mut param_1: u16, mut param_2: u16, mut param_3: u16) -> u16 {
    let mut var2: u16;
    let mut var3: *mut c_char;
    let mut var6: i16;

    let mut var9: *mut c_char;

    // let mut paVar10: *mut Struct825;

    let mut var4 = PTR_LOOP_1050_6066;
    PTR_LOOP_1050_6066 = PTR_LOOP_1050_1000;
    let mut var8 = mem_1000_167a(param_2, param_3);
    PTR_LOOP_1050_6066 = var4;
    if (param_2 | var8) != 0 {
        return var8;
    }
    let mut var10 = CONCAT22(ctx.ES_REG, param_1);
    pass1_1000_25a8();
    pass1_1000_2913(param_1);
    let mut var5 = poss_str_op_1000_28dc(var10);
    if var5.is_null() == false {
        var6 = 0x9;
        if *var5 == 'M' as i8 {
            var6 = 0xf;
        }
        var5 = var5 + var6;
        var6 = 0x22;
        var9 = var5;
        loop {
            if var6 == 0 {
                break;
            }
            var6 += -0x1;
            var3 = var9;
            var9 = var9 + 1;
            if *var3 == '\r' as i8 {
                break;
            }
        }
        var9[-0x1] = '\0';
    }
    FatalAppExit16(0, var5);
    FatalExit();
    var8 = PTR_LOOP_1050_63fe;
    loop {
        let mut var1 = var8;
        var8 = var8 + 1;
        var2 = var1;
        let mut var7 = var8;
        var7 = var2 + 1;
        if var2 == ctx.BP_REG || var7.is_null() {
            return var7;
        }
        var6 = -0x1;
        loop {
            if var6 == 0 {
                break;
            }
            var6 += -0x1;
            var1 = var8;
            var8 = (var8 + 1);
            if var1 == '\0' {
                break;
            }
        }
    }
}

pub unsafe fn pass1_1000_29dc(mut param_1: u16) -> u16 {
    if ___EXPORTEDSTUB != 0xb8 {
        return DAT_1050_1050;
    }
    return uRam100029ed;
}


pub unsafe fn fatal_app_exit_1000_3e9e() {
    FatalAppExit16(0x0, s_ABNORMAL_PROGRAM_TERMINATION_1050_6544);
    return;
}


pub unsafe fn pass1_1000_25d2(
    mut param_1: i16,
    mut param_2: i16,
    fn_ptr_param_3: code2,
    mut param_4: i16,
) -> u16 {
    let mut var1: *mut i16;
    let mut var2: *mut c_char;
    // let mut pstruct_d_var4: *mut StructD;
    let mut var8: *mut i16;
    let mut string9: *mut c_char;
    let mut var3 = (param_1 + 0x1 & 0xfffe);

    let mut var5 = 0u16;
    let mut pstruct_d_var4 = -(var3 - &param_2);
    if (var3 < &param_2) && (PTR_LOOP_1050_000a <= pstruct_d_var4.address_offset_field_0x0)
    {
        if pstruct_d_var4.address_offset_field_0x0 < PTR_LOOP_1050_000c {
            PTR_LOOP_1050_000c = pstruct_d_var4.address_offset_field_0x0;
        }
        // WARNING: Could not recover jumptable at 0x100025f0. Too many branches
        // WARNING: Treating indirect jump as call
        var5 = fn_ptr_param_3();
        return var5;
    }
    let mut paVar10 = (param_2 << 0x10);
    let mut offset7 = 0;
    pass1_1000_25a8();
    pass1_1000_2913(offset7);
    let mut string6 = poss_str_op_1000_28dc(paVar10);
    if string6.is_null() == false {
        offset7 = 0x9;
        if *string6 == 'M' as i8 {
            offset7 = 0xf;
        }
        string6 = string6 + offset7;
        offset7 = 0x22;
        string9 = string6;
        loop {
            if offset7 == 0 {
                break;
            }
            offset7 += -0x1;
            var2 = string9;
            string9 = string9 + 1;
            if *var2 == '\r' as c_char {
                break;
            }
        }
        string9[-0x1] = '\0';
    }
    // FatalAppExit16( 0x0, CONCAT22(0x1050, pcVar6));
    FatalAppExit16(0x0, string6);
    FatalExit();
    var5 = PTR_LOOP_1050_63fe;
    loop {
        var1 = var5;
        var5 = var5 + 1;
        offset7 = *var1;
        var8 = var5;
        if (offset7 == param_4) || (var8 = (offset7 + 1), var8.is_null()) {
            return var8;
        }
        offset7 = -0x1;
        loop {
            if (offset7 == 0) {
                break;
            }
            offset7 += -0x1;
            var1 = var5;
            var5 = (var5 + 1);
            if var1 == '\0' {
                break;
            }
        }
    }
}

pub unsafe fn enum_child_windows_1010_01be() {
    if (PTR_LOOP_1050_0010.is_null()) {
        let func = MakeProcInstance16(HINSTANCE16_1050_038c, block_1010_0000::win_ui_op_1010_0240);
        EnumChildWindows1(0x0, func, func);
        FreeProcInstance16(func);
    }
    return;
}


pub unsafe fn make_proc_inst_1038_cf6c(param_1: *mut u8, param_2: *mut astruct_831)

{
  let mut iVar1: *mut astruct_831;
  let mut uVar1: u16;
// pub unsafe fn *pvVar1;

  uVar1 = (param_2 >> 0x10);
  iVar1 = param_2;
  param_2 = 0x389a;
  iVar1.field2_0x2 = 0x1008;
  iVar1.field3_0x4 = 0;
  iVar1.field5_0x8 = 0;
  param_2 = 0xd23e;
  iVar1.field2_0x2 = &u16_1050_1038;
  _u16_1050_5bc8 = param_2;
  pvVar1 = MakeProcInstance16(HINSTANCE16_1050_038c,&u16_1038_d116);
  iVar1.field3_0x4 = pvVar1;
  iVar1.field4_0x6 = (pvVar1 >> 0x10);
  pvVar1 = MakeProcInstance16(HINSTANCE16_1050_038c,&PTR_LAB_1038_d08b_1_1038_d01e);
  u16_1050_5bcc = pvVar1;
  u16_1050_5bce = (pvVar1 >> 0x10);
  return;
}

pub unsafe fn mixed_ui_op_1020_179c(
    mut param_1: u16,
    mut param_2: u16,
    structb_param_1: *mut StructB,
) {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut pvVar1: LPVOID = null_mut();
    let mut IVar1: i16;
    let mut iVar2: i16;
    let mut puVar11: *mut astruct_816;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut x: i16;
    let mut y_offset: i16;
    let mut IVar2: i16;
    let mut HVar3: HWND16;
    let mut uVar8: u16;
    let mut puVar4: *mut u8;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut in_register_0000000a: u16;
    let mut paVar8: *mut Struct57;
    let mut struct_9: *mut StructB;
    let mut iVar9: i16;
    let mut uVar10: *mut StructB;
    let mut uVar13: u16;
    let mut uVar11: u16;
    let mut uVar14: u16;
    let mut puVar10: *mut u32;
    let mut in_stack_0000fd2a: u16;
    let mut in_stack_0000fd2e: u16;
    let mut in_stack_0000fd30: u16;
    let mut in_stack_0000fe4e: u16;
    let mut in_stack_0000fe52: u16;
    let mut in_stack_0000fe54: u16;
    let mut in_stack_0000fe58: u16;
    let mut in_stack_0000fe5a: u16;
    let mut in_stack_0000fe5c: u16;
    let mut in_stack_0000fe5e: u16;
    let mut uVar16: u16;
    let mut in_buffer_4: *mut c_char;
    let mut in_stack_0000fe88_00: u16;
    let mut in_buf_len_5: i16;
    let mut x_6e: i16;
    let mut y: i16;
    let mut x106: u16;
    let mut y4: i16;
    let mut hwnd_10: HWND16;
    let mut cx: i16;
    let mut cy: i16;
    let mut uStack78: u16;
    let mut uStack76: u16;
    let mut uStack74: u32;
    let mut HStack70: HWND16;
    let mut uStack68: u32;
    let mut uStack64: u32;
    // pub unsafe fn *pvStack60;
    let mut uStack56: u16;
    let mut ppcStack54: *mut *mut c_char;
    let mut uStack50: u32;
    let mut local_2e: *mut astruct_92;
    let mut local_1c: RECT16;
    let mut iStack22: i16;
    let mut iStack20: i16;
    let mut iStack18: i16;
    let mut puStack16: *mut u32;
    let mut pIStack12: *mut INT16 = null_mut();
    let mut uStack8: u16;
    let mut puStack6: *mut u32;
    let mut puVar1: *mut u32;
    let mut puVar2: u32;
    let mut uVar12: u16;
    let mut in_resc_id_3: *mut u8;
    let mut uVar15: u16;
    let mut in_stack_0000fe88: u16;
    let mut uVar9: u32;
    let mut fn_ptr_1: *mut *mut code;

    paVar8 = CONCAT22(in_register_0000000a, param_1);
    dialog_ui_fn_1040_78e2(structb_param_1);
    uVar15 = 0x89;
    puStack6 = mixed_1010_20ba(
        paVar8,
        _u16_1050_0ed0,
        0x890009,
        in_stack_0000fd2e,
        in_stack_0000fe52,
        in_stack_0000fe58,
        in_stack_0000fe5c,
    );
    paVar8 = (paVar8 & 0xffff0000 | puStack6 >> 0x10);
    uVar1 = pass1_1010_65d0(puStack6, uVar15);
    uStack8 = (uVar1 == 0);
    uVar2 = pass1_1010_65d0(puStack6, 0x86);
    if (uVar2 != 0) {
        uStack8 = 0;
    }
    puVar10 = mixed_1010_20ba(
        paVar8,
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000fe88_00, 0x39),
        in_stack_0000fd30,
        in_stack_0000fe54,
        in_stack_0000fe5a,
        in_stack_0000fe5e,
    );
    paVar8 = (paVar8 & 0xffff0000 | puVar10 >> 0x10);
    pvVar1 = puVar10;
    uVar10 = (structb_param_1 >> 0x10);
    struct_9 = structb_param_1;
    struct_9[0x7].field1_0x2 = pvVar1;
    HVar3 = (puVar10 >> 0x10);
    struct_9[0x7].hwnd_0x6 = HVar3;
    uVar16 = struct_9[0x7].field1_0x2;
    fn_ptr_1 = (*&struct_9[0x7].field1_0x2 + 0x10);
    (**fn_ptr_1)(0x1010, uVar16, HVar3, uStack8);
    mem_op_1000_179c(0x12, paVar8);
    uStack76 = paVar8;
    puVar4 = (uStack76 | pvVar1);
    paVar8 = (paVar8 & 0xffff0000 | ZEXT24(puVar4));
    uStack78 = pvVar1;
    if (puVar4.is_null()) {
        struct_9[0x7].lpvoid_field_0x8 = 0;
    } else {
        pass1_1020_1eea(
            puVar4,
            CONCAT22(uStack76, pvVar1),
            structb_param_1,
            struct_9.lpvoid_field_0x8,
        );
        struct_9[0x7].lpvoid_field_0x8 = pvVar1;
        struct_9[0x7].max_count_field_0x10 = paVar8;
    }
    puVar1 = &struct_9[0x7].field1_0x2;
    pIStack12 = (puVar1 & 0xffff0000 | (puVar1 + 0xa));
    puStack16 = mixed_1010_20ba(
        paVar8,
        _u16_1050_0ed0,
        CONCAT22(uVar16, 0x48),
        in_stack_0000fd2a,
        in_stack_0000fe4e,
        in_stack_0000fe54,
        in_stack_0000fe58,
    );
    GetClientRect16(&local_1c, &DAT_1050_1050);
    IVar1 = GetSystemMetrics16(SM_CYCAPTION);
    uVar13 = (pIStack12 >> 0x10);
    iVar9 = pIStack12;
    (iVar9 + 0x6) = IVar1 + iStack22;
    uVar11 = (puStack16 >> 0x10);
    iStack18 = (puStack16 + 0xa);
    iStack20 = (puStack16 + 0xc);
    (iVar9 + 0x2) = (iStack20 - (iVar9 + 0x6)) / 0x2;
    iVar2 = iStack18 - (iVar9 + 0x4);
    uVar5 = iVar2 >> 0xf;
    uVar9 = uVar5;
    *pIStack12 = iVar2 / 0x2;
    pass1_1028_dc52(CONCAT22(0x1050, &local_2e), 0x1, 0x0, 0x100);
    uStack56 = 0;
    loop {
        uVar6 = uVar9;
        puVar11 = &local_2e;
        pass1_1028_e4ec(CONCAT22(0x1050, puVar11));
        uStack50 = CONCAT22(uVar6, puVar11);
        uVar7 = uVar6 | puVar11;
        uVar9 = uVar7;
        if (uVar7 == 0) {
            break;
        }
        ppcStack54 = puVar11.field16_0x10;
        if (ppcStack54.is_null() == false) {
            pass1_1000_3cea(
                structb_param_1 & 0xffff0000 | ZEXT24(&struct_9.field8_0x10),
                *ppcStack54,
            );
        }
    }
    uVar3 = pass1_1020_1da8(puVar11, 0x0, structb_param_1);
    struct_9[0x7].field5_0xa = uVar3;
    uVar4 = pass1_1010_65d0(puStack6, 0x86);
    if ((uVar4 == 0) || (struct_9[0x7].field5_0xa.is_null() == false)) {
        puVar2 = &struct_9[0x7].field1_0x2;
        (puVar2 + 0x2c) = 0;
        hwnd_10 = GetDlgItem16(0x175, struct_9.lpvoid_field_0x8);
        if (uStack8 != 0) {
            load_string_1010_84e0(
                _u16_1050_14cc,
                (_u16_1050_14cc >> 0x10),
                0x100,
                &stack0xfe88,
                &DAT_1050_1050,
            );
            SetWindowText16(CONCAT13(0x10, CONCAT12(0x50, &stack0xfe88)), hwnd_10);
        }
        pvStack60 = MakeProcInstance16(HINSTANCE16_1050_038c, destroy_win_1020_1e1e);
        GetWindowRect16(CONCAT22(0x1050, &x_6e), hwnd_10);
        cx = _x106 - x_6e;
        cy = y4 - y;
        x = cx - (pIStack12 + 0x4);
        y_offset = GetSystemMetrics16(SM_CYCAPTION);
        MoveWindow16(0x0, cy, cx, y - y_offset, -x / 0x2, hwnd_10);
    } else {
        win_1008_5c7c(uVar4, uVar7, _u16_1050_02a0, 0x9d0001);
        (struct_9 + 0x7).field0_0x0 = uVar4;
        pvStack60 = MakeProcInstance16(HINSTANCE16_1050_038c, destroy_win_1020_1dea);
    }
    EnumChildWindows1(0x0, pvStack60, struct_9.lpvoid_field_0x8);
    FreeProcInstance16(pvStack60);
    HStack70 = GetDlgItem16(0x1, struct_9.lpvoid_field_0x8);
    GetWindowRect16(CONCAT22(0x1050, &local_1c), HStack70);
    uStack64 = _param_2;
    local_1c.x = _param_2 - local_1c.x;
    uStack74 = CONCAT22(local_1c.x, iStack22 - local_1c.y);
    uStack68 = local_1c & 0xffff0000 | (-(local_1c.x - (pIStack12 + 0x4)) / 0x2);
    IVar2 = GetSystemMetrics16(SM_CYCAPTION);
    uStack68 = uStack68 & 0xffff | (uStack68 - IVar2) << 0x10;
    if (struct_9[0x7].field5_0xa.is_null()) {
        //    if (uStack8 == 0) goto LAB_1020_1b24;
        in_buf_len_5 = 0x72e;
        in_resc_id_3 = &stack0xfe88;
        in_buffer_4 = &DAT_1050_1050;
    } else {
        load_string_1010_84e0(
            _u16_1050_14cc,
            (_u16_1050_14cc >> 0x10),
            0x100,
            &stack0xfe88,
            &DAT_1050_1050,
        );
        HVar3 = GetDlgItem16(0x175, struct_9.lpvoid_field_0x8);
        SetWindowText16(CONCAT22(0x1050, &stack0xfe88), HVar3);
        in_buffer_4 = &stack0xfe88;
        in_buf_len_5 = &DAT_1050_1050;
        in_resc_id_3 = 0x3ff;
    }
    load_string_1010_84e0(
        _u16_1050_14cc,
        (_u16_1050_14cc >> 0x10),
        in_resc_id_3,
        in_buffer_4,
        in_buf_len_5,
    );
    SetWindowText16(CONCAT22(0x1050, &stack0xfe88), HStack70); //
    // LAB_1020_1b24:
    MoveWindow16(
        0x0,
        uStack74,
        (uStack74 >> 0x10),
        uStack68,
        uStack68,
        HStack70,
    );
    uVar14 = (pIStack12 >> 0x10);
    iVar2 = pIStack12;
    SetWindowPos16(
        0x44,
        (iVar2 + 0x6),
        (iVar2 + 0x4),
        (iVar2 + 0x2),
        *pIStack12,
        0x0,
        struct_9.lpvoid_field_0x8,
    );
    return;
}


pub unsafe fn mixed_struct_op_1040_8fb8
               (mut param_1: u32,param_2: *mut astruct_65,mut param_3: u16 ,param_4: *mut c_char,mut param_5: u16 ,mut param_6: u16 ,mut param_7: u16 ,
               mut param_8: u16 ,mut param_9: u16 ,mut param_10: u16 )

{
  let mut uVar5: u16;
  let mut uVar3: u16;
  let mut uVar6: u16;
  let mut iVar3: *mut astruct_65;
  let mut uVar4: *mut astruct_65;
  let mut unaff_CS: u16;
  let mut uVar1: u16;
  let mut uVar2: u16;

  uVar4 = (param_2 >> 0x10);
  iVar3 = param_2;
  param_2.field0_0x0 = 0x389a;
  iVar3.field1_0x2 = 0x1008;
  iVar3.field2_0x4 = 0;
  iVar3.field4_0x8 = 0;
  iVar3.field6_0xc = 0;
  iVar3.field8_0x10 = 0;
  iVar3.field10_0x14 = 0;
  iVar3.field11_0x18 = 0;
  iVar3.field12_0x1a = param_9;
  iVar3.field13_0x1c = param_8;
  iVar3[0x1].field8_0x10 = 0x5;
  iVar3[0x1].field9_0x12 = 0;
  iVar3[0x1].field10_0x14 = 0;
  (&iVar3[0x1].field10_0x14 + 0x2) = 0x2;
  iVar3[0x1].field11_0x18 = 0;
  iVar3[0x1].field12_0x1a = param_3;
  param_2.field0_0x0 = 0x9800;
    // this is probably just the value 0x1040
  iVar3.field1_0x2 = &PTR_LOOP_1050_1040;
  uVar1 = iVar3[0x1].field8_0x10;
  iVar3[0x1].field1_0x2 = uVar1;
  (iVar3 + 1).field0_0x0 = uVar1;
  iVar3[0x1].field3_0x6 = 0;
  iVar3[0x1].field2_0x4 = 0;
  if ((param_7 != 0) && (param_6 != 0)) {
    iVar3[0x1].field9_0x12 = 0x1;
    uVar5 = FUN_1010_830a(0x0,param_1,unaff_CS,_u16_1050_14cc,param_7);
    iVar3.field4_0x8 = uVar5;
    iVar3.field5_0xa = param_1;
    uVar5 = FUN_1010_830a(uVar5,param_1,0x1010,_u16_1050_14cc,param_6);
    iVar3.field6_0xc = uVar5;
    iVar3.field7_0xe = param_1;
    if (param_5 == 0) {
      iVar3.field8_0x10 = 0;
    }
    else {
      uVar5 = FUN_1010_830a(uVar5,param_1,0x1010,_u16_1050_14cc,param_5);
      iVar3.field8_0x10 = uVar5;
      iVar3.field9_0x12 = param_1;
    }
  }
  uVar6 = param_1;
  uVar2 = iVar3[0x1].field8_0x10;
  iVar3[0x1].field5_0xa = uVar2;
  iVar3[0x1].field4_0x8 = uVar2;
  iVar3[0x1].field6_0xc = 0;
  if (param_4.is_null() == false) {
    uVar3 = str_op_1008_60e8(uVar6,param_4);
    iVar3.field2_0x4 = uVar3;
    iVar3.field3_0x6 = uVar6;
  }
  iVar3.field16_0x22 = 0;
  iVar3.field14_0x1e = 0;
  iVar3.field15_0x20 = 0;
  if (_u16_1050_5e18.is_null()) {
    _u16_1050_5e18 = MakeProcInstance16(HINSTANCE16_1050_038c,call_win_proc_1040_9684);
  }
  PTR_LOOP_1050_5e16 = PTR_LOOP_1050_5e16 + 1;
  return;
}


pub unsafe fn make_proc_inst_1040_a234(param_1: *mut u8, param_2: *mut u8, mut param_3: u16, mut param_4: u32)

{
  block_1040_a000::pass1_1040_b040(CONCAT22(param_2, param_1), CONCAT22(param_4, param_3), (param_4 >> 0x10));
  CONCAT22(param_2,param_1) = 0xa4e8;
  (param_1 + 0x2) = &PTR_LOOP_1050_1040;
  if (_PTR_LOOP_1050_5edc.is_null()) {
    _PTR_LOOP_1050_5edc = MakeProcInstance16(HINSTANCE16_1050_038c, block_1040_a000::call_win_proc_1040_a40e);
  }
  (param_1 + 0xc) = _PTR_LOOP_1050_5edc;
  PTR_LOOP_1050_5eda = PTR_LOOP_1050_5eda + 1;
  PTR_LOOP_1050_5ee0 = param_1;
  PTR_LOOP_1050_5ee2 = param_2;
  return;
}


pub unsafe fn free_proc_inst_1038_cfda(param_1: *mut StructD)

{
  let mut iVar1: *mut StructD;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.address_offset_field_0x0 = 0xd23e;
  iVar1.address_offset_field_0x2 = &u16_1050_1038;
  FreeProcInstance16(&iVar1.hfile_0x4);
  FreeProcInstance16(_u16_1050_5bcc);
  iVar1.hfile_0x4 = 0;
  param_1.address_offset_field_0x0 = 0x389a;
  iVar1.address_offset_field_0x2 = 0x1008;
  return;
}


pub unsafe fn mix_win_ui_op_1040_911e(param_1: *mut StructD)

{
  let mut puVar3: *mut u32;
  let mut struct_1: *mut StructD;
  let mut uVar5: u16;
  let mut puVar1: *mut u32;
  let mut puVar2: *mut u32;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut fn_ptr_1: *mut *mut code;

  uVar5 = (param_1 >> 0x10);
  struct_1 = param_1;
  param_1.address_offset_field_0x0 = 0x9800;
  struct_1.address_offset_field_0x2 = &PTR_LOOP_1050_1040;
  if (&struct_1.field_0x38 != 0) {
    puVar1 = struct_1.field5_0x8;
    uVar2 = struct_1.field6_0xa;
    if ((uVar2 | puVar1) != 0) {
      fn_ptr_1 = *puVar1;
      (**fn_ptr_1)();
    }
    puVar3 = struct_1.field7_0xc;
    uVar3 = struct_1.field8_0xe;
    if ((uVar3 | puVar3) != 0) {
      fn_ptr_1 = *puVar3;
      (**fn_ptr_1)();
    }
    puVar3 = &struct_1.field_0x10;
    uVar4 = struct_1.field11_0x12;
    if ((uVar4 | puVar3) != 0) {
      fn_ptr_1 = *puVar3;
      (**fn_ptr_1)();
    }
  }
  fn_ptr_1000_17ce(*&struct_1.hfile_0x4);
  SetWindowLong16(struct_1.field12_0x14,-0x4,struct_1.field13_0x18);
  RemoveProp16(s_thisLo_1050_5e1c,struct_1.field13_0x18);
  RemoveProp16(s_thisHi_1050_5e23,struct_1.field13_0x18);
  RemoveProp16(s_procLo_1050_5e2a,struct_1.field13_0x18);
  RemoveProp16(s_procHi_1050_5e31,struct_1.field13_0x18);
  RemoveProp16(s_IsDlg_1050_5e38,struct_1.field13_0x18);
  PTR_LOOP_1050_5e16 = PTR_LOOP_1050_5e16 -0x1;
  if (PTR_LOOP_1050_5e16.is_null()) {
    FreeProcInstance16(_u16_1050_5e18);
    _u16_1050_5e18 = null_mut();
  }
  param_1.address_offset_field_0x0 = 0x389a;
  struct_1.address_offset_field_0x2 = 0x1008;
  return;
}


pub unsafe fn free_proc_inst_1040_a294(param_1: *mut StructD)

{
  let mut in_stack_0000ffde: u16;

  param_1.address_offset_field_0x0 = 0xa4e8;
  (param_1 + 0x2) = &PTR_LOOP_1050_1040;
  PTR_LOOP_1050_5eda = PTR_LOOP_1050_5eda -0x1;
  if (PTR_LOOP_1050_5eda.is_null()) {
    FreeProcInstance16(_PTR_LOOP_1050_5edc);
    _PTR_LOOP_1050_5edc = null_mut();
  }
  unk_draw_op_1040_b0f8(in_stack_0000ffde,param_1);
  return;
}


pub unsafe fn mixed_win_op_1008_0c60(
    mut param_1: u16,
    param_2: *mut StructD,
    mut param_3: u16,
    mut param_4: u16,
    param_5: *mut Struct72,
    mut param_6: u16,
    mut param_7: u16,
    mut param_8: u16,
) -> BOOL16 {
    let mut ppcVar1: *mut *mut code;
    let mut HVar2: HISTANCE16;
    let mut iVar3: i16;
    let mut BVar4: bool;
    let mut uVar7: u16;
    let mut pSVar8: *mut StructD;
    let mut uVar15: u16;
    let mut struct_var5: *mut Struct72;
    let mut uVar6: u16;
    let mut in_AF: u8;
    let mut uVar5: u32;
    let mut lresult_6: LRESULT;
    let mut pcVar16: *mut c_char;
    let mut puVar17: *mut u32;
    let mut in_stack_0000fcd2: u16;
    let mut in_stack_0000fce4: u16;
    let mut in_stack_0000fcf8: u16;
    let mut in_stack_0000fd58: *mut WNDCLASS16;
    let mut in_stack_0000fe34: u16;
    let mut in_stack_0000fe3a: u16;
    let mut in_stack_0000ff58: u16;
    let mut in_stack_0000ff5e: u16;
    let mut in_stack_0000ff62: u16;
    let mut in_stack_0000ff64: u16;
    let mut in_stack_0000ff68: u16;
    let mut WVar18: WPARAM16;
    let mut uVar19: u16;
    let mut local_64: [u8; 0x50] = [0; 0x50];
    let mut uStack20: u32;
    let mut HStack16: HCURSOR16;
    let mut HStack14: HCURSOR16;
    let mut uStack6: u32;
    let mut puVar1: *mut u32;
    let mut uVar9: u8;
    let mut uVar10: u8;
    let mut iVar12: i16;
    let mut uVar13: u16;
    let mut struct_var15: *mut Struct72;
    let mut uVar14: u16;
    let mut in_stack_0000ff92: u16;
    let mut uVar11: u8;
    let mut uVar12: u16;
    let mut paVar9: *mut Struct57;

    struct_var5 = param_5;
    uVar6 = (param_5 >> 0x10);
    pSVar8 = param_2;

    //   switch(param_6) {
    match param_6 {
        // case 0x64:
        0x64 => {
            BVar4 = pass1_1008_07d8(param_1, param_2, struct_var5);
            win_ui_cursor_op_1008_2e9a(
                param_2,
                param_5,
                in_stack_0000fd58,
                in_stack_0000fcd2,
                in_stack_0000fce4,
                in_stack_0000fcf8,
            );
            return BVar4;
        }
        //   case 0x65:
        0x65 => {
            pass1_1008_3018(pSVar8, param_5);
            return param_1;
        }
        //   case 0x66:
        0x66 => {
            pass1_1008_30cc(param_1, pSVar8, param_5);
            return param_1;
        }
        //   case 0x67:
        0x67 => {
            iVar3 = win_ui_op_1008_2b54(param_1, pSVar8, param_5);
            if (iVar3 == 0) {
                return 0x0;
            }
        }
        //   case 0xee:
        0xee => {
            uVar19 = struct_var5.field7_0x8;
            WVar18 = 0;
            uVar9 = '\x10';
            uVar10 = '\0';
        }
        // TODO: goto LAB_1008_0d18;
        //   case 0x68:
        0x68 => {
            pass1_1030_8344(_u16_1050_5748, 0x4000001);
            uVar7 = param_2 | param_1;
            paVar9 = (param_2 & 0xffff0000 | uVar7);
            if (uVar7 == 0) {
                return param_1;
            }
            if (PTR_LOOP_1050_4fe8.is_null() == false) {
                pcVar16 = load_string_1010_847e(_u16_1050_14cc, 0x57b);
                BVar4 = MessageBox16(
                    0x10,
                    pcVar16,
                    s_You_may_not_run_a_turn__The_game_1050_0172,
                    struct_var5.field7_0x8,
                );
                return BVar4;
            }
            HStack14 = LoadCursor16(0x7f02, 0x0);
            HStack16 = SetCursor16(HStack14);
            puVar17 = mixed_1010_20ba(
                paVar9,
                _u16_1050_0ed0,
                CONCAT22(param_3, 0x29),
                in_stack_0000fe3a,
                in_stack_0000ff5e,
                in_stack_0000ff64,
                in_stack_0000ff68,
            );
            uVar15 = (paVar9 >> 0x10);
            uStack20 = SUB42(puVar17, 0x0);
            uStack20 = (puVar17 >> 0x10);
            pass1_1018_262e(puVar17);
            pass1_1030_838e(_u16_1050_5748);
            (_u16_1050_5748 + 0x8) = 0x1;
            pass1_1030_8326();
            pcVar16 = load_string_1010_847e(_u16_1050_14cc, 0x5dc);
            paVar9 = CONCAT22(uVar15, (pcVar16 >> 0x10));
            sys_1000_3f9c(
                CONCAT13(0x10, CONCAT12(0x50, local_64)),
                s__s__ld_1050_019c,
                pcVar16,
            );
            uVar12 = 0;
            ppcVar1 = (param_5 + 0x14);
            (**ppcVar1)(
                0x0,
                struct_var5,
                (param_5 >> 0x10),
                0x0,
                local_64,
                &DAT_1050_1050,
            );
            puVar17 = mixed_1010_20ba(
                paVar9,
                _u16_1050_0ed0,
                CONCAT22(uVar12, 0x37),
                in_stack_0000fe34,
                in_stack_0000ff58,
                in_stack_0000ff5e,
                in_stack_0000ff62,
            );
            pass1_1008_a9ec(puVar17);
            SetCursor16(HStack16);
            uVar19 = struct_var5.field7_0x8;
            WVar18 = 0xfc;
            uVar11 = '\x11';
        }
        // TODO: goto LAB_1008_0e3d;
        //   _ =>
        _ => {
            if (((&struct_var5.field227_0xe8 + 0x2) | &struct_var5.field227_0xe8) == 0) {
                return 0x0;
            }
            puVar1 = struct_var5.field227_0xe8;
            ppcVar1 = (*struct_var5.field227_0xe8 + 0x40);
            BVar4 = (**ppcVar1)(0x8, puVar1, (puVar1 >> 0x10), param_6);
            return BVar4;
        }
        //   case 0x6e:
        0x6e => {
            iVar12 = 0x2;
        }
        // TODO: goto LAB_1008_0cba;
        //   case 0x6f:
        0x6f => {
            uStack6 = FUN_1010_830a(param_1, param_2, 0x1008, _u16_1050_14cc, 0x1f8);
            uStack6 = SUB42(param_2, 0x0);
            BVar4 = WinHelp16(0x0, 0x3, CONCAT22(uStack6, uStack6), struct_var5.field7_0x8);
            return BVar4;
        }
        //   case 0x70:
        0x70 => {
            iVar12 = 0x1; //
                          // LAB_1008_0cba:
            uVar5 = pass1_1038_af40(
                struct_var5,
                pSVar8,
                _PTR_LOOP_1050_5b7c,
                struct_var5.field7_0x8,
                iVar12,
            );
            return uVar5;
        }
        //   case 0x71:
        0x71 => {
            HVar2 = WinExec16(0x3, s_notepad_read_me_1050_0162);
            return HVar2;
        }
        //   case 0x79:
        0x79 => {
            BVar4 = post_msg_1008_2d22(param_5);
            return BVar4;
        }
        //   case 0x7a:
        0x7a => {
            uVar13 = 0xb;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0x7b:
        0x7b => {
            uVar13 = 0x1e;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0x7c:
        0x7c => {
            uVar13 = 0x1f;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0x7d:
        0x7d => {
            uVar13 = 0x21;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0x7e:
        0x7e => {
            uVar13 = 0x35;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0x7f:
        0x7f => {
            uVar14 = 0x39;
        }
        // break;
        //   case 0x80:
        0x80 => {
            uVar13 = 0x22;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0x81:
        0x81 => {
            uVar14 = 0x36;
        }
        // break;
        //   case 0x82:
        0x82 => {
            uVar14 = 0x37;
        }
        // break;
        //   case 0x83:
        0x83 => {
            uVar14 = 0x38;
        }
        // break;
        //   case 0x84:
        0x84 => {
            uVar14 = 0x3a;
        }
        // break;
        //   case 0x85:
        0x85 => {
            uVar14 = 0x3b;
        }
        // break;
        //   case 0x86:
        0x86 => {
            uVar14 = 0x3c;
        }
        // break;
        //   case 0x87:
        0x87 => {
            uVar14 = 0x3d;
        }
        // break;
        //   case 0x88:
        0x88 => {
            uVar14 = 0x3e;
        }
        // break;
        //   case 0x89:
        0x89 => {
            uVar14 = 0x3f;
        }
        // break;
        //   case 0x8a:
        0x8a => {
            uVar14 = 0x40;
        }
        // break;
        //   case 0x8b:
        0x8b => {
            uVar13 = 0xc;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0x8c:
        0x8c => {
            uVar14 = 0x41;
        }
        // break;
        //   case 0x8d:
        0x8d => {
            uVar14 = 0x42;
        }
        // break;
        //   case 0x8e:
        0x8e => {
            uVar14 = 0x43;
        }
        // break;
        //   case 0x8f:
        0x8f => {
            uVar14 = 0x44;
        }
        // break;
        //   case 0x90:
        0x90 => {
            uVar14 = 0x45;
        }
        // break;
        //   case 0x91:
        0x91 => {
            uVar14 = 0x46;
        }
        // break;
        //   case 0x92:
        0x92 => {
            uVar14 = 0x47;
        }
        // break;
        //   case 0x93:
        0x93 => {
            uVar13 = 0x23;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0x94:
        0x94 => {
            uVar13 = 0x24;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0x95:
        0x95 => {
            uVar14 = 0x48;
        }
        // break;
        //   case 0x96:
        0x96 => {
            uVar14 = 0x49;
        }
        // break;
        //   case 0x97:
        0x97 => {
            uVar14 = 0x4a;
        }
        // break;
        //   case 0x98:
        0x98 => {
            uVar14 = 0x4b;
        }
        // break;
        //   case 0x99:
        0x99 => {
            uVar14 = 0x4c;
        }
        // break;
        //   case 0x9a:
        0x9a => {
            uVar13 = 0xd;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0x9b:
        0x9b => {
            uVar14 = 0x4d;
        }
        // break;
        //   case 0x9c:
        0x9c => {
            uVar14 = 0x4e;
        }
        // break;
        //   case 0x9d:
        0x9d => {
            uVar14 = 0x4f;
        }
        // break;
        //   case 0x9e:
        0x9e => {
            uVar14 = 0x50;
        }
        // break;
        //   case 0x9f:
        0x9f => {
            uVar14 = 0x51;
        }
        // break;
        //   case 0xa0:
        0xa0 => {
            uVar13 = 0xe;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xa1:
        0xa1 => {
            uVar13 = 0xf;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xa2:
        0xa2 => {
            uVar14 = 0x52;
        }
        // break;
        //   case 0xa3:
        0xa3 => {
            uVar13 = 0x10;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xa4:
        0xa4 => {
            uVar14 = 0x53;
        }
        // break;
        //   case 0xa5:
        0xa5 => {
            uVar13 = 0x11;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xa6:
        0xa6 => {
            uVar13 = 0x12;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xa7:
        0xa7 => {
            uVar14 = 0x57;
        }
        // break;
        //   case 0xa8:
        0xa8 => {
            uVar13 = 0x13;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xa9:
        0xa9 => {
            uVar13 = 0x14;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xaa:
        0xaa => {
            uVar14 = 0x58;
        }
        // break;
        //   case 0xab:
        0xab => {
            uVar14 = 0x63;
        }
        // break;
        //   case 0xac:
        0xac => {
            uVar14 = 0x59;
        }
        // break;
        //   case 0xad:
        0xad => {
            uVar14 = 0x5a;
        }
        // break;
        //   case 0xae:
        0xae => {
            uVar14 = 0x5b;
        }
        // break;
        //   case 0xaf:
        0xaf => {
            uVar14 = 0x15;
        }
        // break;
        //   case 0xb0:
        0xb0 => {
            uVar13 = 0x25;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xb1:
        0xb1 => {
            uVar14 = 0x5c;
        }
        // break;
        //   case 0xb2:
        0xb2 => {
            uVar14 = 0x16;
        }
        // break;
        //   case 0xb3:
        0xb3 => {
            uVar14 = 0x5d;
        }
        // break;
        //   case 0xb4:
        0xb4 => {
            uVar13 = 0x5e;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xb5:
        0xb5 => {
            uVar14 = 0x5f;
        }
        // break;
        //   case 0xb6:
        0xb6 => {
            uVar14 = 0x60;
        }
        // break;
        //   case 0xb7:
        0xb7 => {
            uVar14 = 0x61;
        }
        // break;
        //   case 0xb8:
        0xb8 => {
            uVar14 = 0x62;
        }
        // break;
        //   case 0xb9:
        0xb9 => {
            uVar14 = 0x64;
        }
        // break;
        //   case 0xba:
        0xba => {
            uVar14 = 0x65;
        }
        // break;
        //   case 0xbb:
        0xbb => {
            uVar14 = 0x66;
        }
        // break;
        //   case 0xbc:
        0xbc => {
            uVar14 = 0x67;
        }
        // break;
        //   case 0xbd:
        0xbd => {
            uVar14 = 0x68;
        }
        // break;
        //   case 0xbe:
        0xbe => {
            uVar14 = 0x69;
        }
        // break;
        //   case 0xbf:
        0xbf => {
            uVar13 = 0x17;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xc0:
        0xc0 => {
            uVar13 = 0x18;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xc1:
        0xc1 => {
            uVar13 = 0x19;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xc2:
        0xc2 => {
            uVar13 = 0x1a;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xc3:
        0xc3 => {
            uVar13 = 0x1b;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xc4:
        0xc4 => {
            uVar13 = 0x1c;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xc5:
        0xc5 => {
            uVar13 = 0x1d;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xc6:
        0xc6 => {
            uVar13 = 0x4;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xc8:
        0xc8 => {
            uVar13 = 0x3;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xc9:
        0xc9 => {
            uVar13 = 0x1;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xca:
        0xca => {
            uVar13 = 0x5;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xcb:
        0xcb => {
            pass1_1008_087e(in_AF, param_1, pSVar8);
            uVar13 = 0x6;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xcc:
        0xcc => {
            uVar13 = 0x7;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xcd:
        0xcd => {
            uVar13 = 0x8;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xce:
        0xce => {
            uVar13 = 0x9;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xcf:
        0xcf => {
            uVar13 = 0xa;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xd0:
        0xd0 => {
            uVar13 = 0x26;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xd1:
        0xd1 => {
            uVar13 = 0x27;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xd2:
        0xd2 => {
            uVar13 = 0x28;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xd3:
        0xd3 => {
            uVar13 = 0x29;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xd4:
        0xd4 => {
            uVar13 = 0x2a;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xd5:
        0xd5 => {
            uVar13 = 0x2b;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xd6:
        0xd6 => {
            uVar13 = 0x2c;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xd7:
        0xd7 => {
            uVar13 = 0xd7;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xd8:
        0xd8 => {
            uVar13 = 0x2e;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xd9:
        0xd9 => {
            uVar13 = 0x2f;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xda:
        0xda => {
            uVar13 = 0x30;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xdb:
        0xdb => {
            uVar13 = 0x31;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xdc:
        0xdc => {
            uVar13 = 0x32;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xdd:
        0xdd => {
            uVar13 = 0x33;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xde:
        0xde => {
            uVar13 = 0x34; //
                           // LAB_1008_0f3e:
            cursor_op_1008_2dcc(param_2, param_5, uVar13, param_7, param_8);
            return param_1;
        }
        //   case 0xdf:
        0xdf => {
            uVar14 = 0x55;
        }
        // break;
        //   case 0xe0:
        0xe0 => {
            uVar14 = 0x56;
        }
        // break;
        //   case 0x100:
        0x100 => {
            win_1008_5c5c(param_1, pSVar8, _u16_1050_02a0, 0x1dc);
            return param_1;
        }
        //   case 0x12c:
        0x12c => {
            uVar19 = struct_var5.field7_0x8;
            WVar18 = 0xf020;
            uVar9 = '\x12';
            uVar10 = '\x01'; //
                             // LAB_1008_0d18:
            lresult_6 = SendMessage16(0x0, WVar18, CONCAT11(uVar10, uVar9), uVar19);
            return lresult_6;
        }
        //   case 0x12e:
        0x12e => {
            uVar19 = struct_var5.field7_0x8;
            WVar18 = 0xf060;
            uVar11 = '\x12'; //
                             // LAB_1008_0e3d:
            BVar4 = PostMessage16(0x0, WVar18, CONCAT11(0x1, uVar11), uVar19);
            return BVar4;
        }
    };
    ui_op_1008_2c4e(pSVar8, param_8, param_5, uVar14);
    return param_1;
}
