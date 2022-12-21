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
