extern crate libc;

mod bad_funcs;
mod big_funcs;
mod bool_funcs;
mod cleanup;
mod draw;
mod err_funcs;
mod exit;
mod file_funcs;
mod func_ptr_funcs;
mod globals;
mod init;
mod list_funcs;
mod mem_funcs;
mod other_funcs;
mod pass2_funcs;
mod pass3_funcs;
mod pass4_funcs;
mod pass5_funcs;
mod pass6_funcs;
mod pass7_funcs;
mod pass8_funcs;
mod pass_funcs;
mod sound_funcs;
mod string_funcs;
mod struct_funcs;
mod sys_funcs;
mod ui_funcs;
mod util;
mod loops;
mod constants;
mod prog_structs;
mod app_context;
mod typedefs;
mod sys_structs;

use defines::{code, AppContext};
use func_ptr_funcs::{call_fn_ptr_1000_24cd, call_fn_ptr_1000_24db};
use mem_funcs::alloc_mem_1000_167a;
use other_funcs::{big_fn_1010_b038, empty_fn_1000_55ac};
use pass_funcs::{
    pass1_1008_57a4, pass1_1008_5b12, pass1_1008_6978, pass1_fn_1000_25a8, pass1_fn_1000_2913,
};
use sound_funcs::mci_fn_1020_08b6;
use string_funcs::process_string_1000_28dc;
use struct_funcs::{process_struct_1000_179c, process_struct_1010_20ba, process_struct_1018_e91e};
use sys_funcs::{FatalAppExit16, FatalExit, GetVersion16, LockSegment16, WaitEvent16, dos3_call_1000_23ea, get_dos_env_1000_27d6, get_module_file_name_1000_262c};
use util::{CONCAT11, CONCAT22};
use crate::app_context::AppContext;
use crate::sys_funcs::{InitTask16, swi, InitApp16, make_htask};
use crate::err_funcs::set_error_mode_1010_8b14;
use crate::ui_funcs::msg_box_1010_8bb4;
use crate::pass_funcs::{pass1_1008_6562, pass1_1008_3f92};
use crate::pass8_funcs::pass1_1010_878c;
use crate::file_funcs::close_file_1008_496c;
use crate::struct_funcs::process_struct_1008_48fe;
use std::intrinsics::offset;
use crate::prog_structs::prog_structs_31::Struct449;
use crate::prog_structs::prog_structs_24::Struct103;

const INT_21: u16 = 0x21;

pub unsafe fn entry(
    ctx: &mut AppContext,
    param_1: &mut String,
    param_2: &mut u16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
) -> i32 {
    let pi32_a: String;
    let fn_ptr_b: fn();
    let mut i16_a: i16;
    let mut fn_ptr_d: fn();
    // let mut string_e: u16;
    let mut offset_f: u16;
    let mut pi32_g: String;
    let mut i32_h: i32;
    let mut pu8_j: u16 = 0;
    let mut pc_k: String;
    let mut pu8_m: u16 = 0;
    // let mut string_n: u16;
    let mut pu8_p: u16 = 0;
    let mut b_q: bool;
    let mut win_version_r: u32;
    let mut u32_s: u32;
    let mut u32_t: u32;
    let mut i32_v: i32;
    let mut fn_ptr_w: u32;
    let mut string_x: String;

    u32_s = CONCAT22(param_5, ctx.g_u16_ptr_1050_5f84);
    loop {
        InitTask16(None);
        ctx.g_u16_ptr_1050_5f84 = u32_s as u16;
        if param_2 != 0
            // TODO
            // && (
            //     b_q = param_4 < 0xff00,
            //     param_4 = param_4 + 0x100,
            //     ctx.PTR_LOOP_1050_5f7e = pu8_p,
            //     b_q,
            // )
        {
            ctx.PTR_LOOP_1050_5f48 = param_4;
            ctx.PTR_LOOP_1050_5f4a = pu8_j;
            ctx.g_h_instance = pu8_m;
            ctx.PTR_LOOP_1050_5f4e = param_3;
            ctx.PTR_LOOP_1050_5f50 = pu8_p;
            LockSegment16(0xffff);
            ctx.PTR_LOOP_1050_5f52 = (u32_s >> 0x10) as u16;
            ctx.g_u16_ptr_1050_5f84 = u32_s as u16;
            win_version_r = GetVersion16();
            ctx.PTR_LOOP_1050_5f52 = (u32_s >> 0x10);
            ctx.g_u16_ptr_1050_5f84 = u32_s as u16;
            ctx.PTR_LOOP_1050_5f80 = win_version_r as u16;
            fn_ptr_d = swi(INT_21);
            u32_t = u32_s;
            // TODO
            //u32_s = fn_ptr_d();
            ctx.PTR_LOOP_1050_5f52 = (u32_t >> 0x10) as u16;
            ctx.g_u16_ptr_1050_5f84 = u32_t as u16;
            ctx._u8_1050_5f82 = u32_s;
            ctx.u8_1050_5f87 = '\0';
            WaitEvent16(make_htask(0));
            ctx.g_u16_ptr_1050_5f84 = u32_s as u16;
            i16_a = InitApp16(ctx.g_h_instance);
            ctx.g_u16_ptr_1050_5f84 = u32_s as u16;
            if i16_a != 0 {
                break;
            }
        }
        *param_2 = call_fn_ptr_1000_24db(ctx);
        ctx.g_u16_ptr_1050_5f84 = u32_s as u16;
    }
    dos3_call_1000_23ea(ctx, i16_a, param_3);
    get_module_file_name_1000_262c(ctx, param_1);
    get_dos_env_1000_27d6(ctx);
    // empty_fn_1000_55ac();
    fn_ptr_d = func_0x100023be(offset);
    call_fn_ptr_1000_24cd(&fn_ptr_d);
    pass1_fn_1000_25a8(ctx);
    pass1_fn_1000_2913(ctx, 0);
    let mut string_e = process_string_1000_28dc(ctx, (ctx.s_version__d__d_1050_0012 + 3));
    if !string_e.is_empty() {
        offset_f = 9;
        if string_e[0] == 'M' {
            offset_f = 0xf;
        }
        string_e = string_e[offset_f..];
        i32_h = 0x22;
        let mut string_n = string_e.clone();
        while {
            if i32_h == 0 {
                break;
            }
            i32_h = i32_h - 1;
            string_x = string_n;
            string_n = string_n[1..].clone();
            *string_x != '\r'
        } {}
        *(string_n - 1) = 0;
    }
    // TODO: make sure we're referencing
    // param_1 = CONCAT22(0x1050, string_1);
    *param_1 = string_e;
    FatalAppExit16(0, param_1);
    FatalExit(0xff);
    pc_k = ctx.s___NMSG___1050_63f6[8..].clone();
    loop {
        pi32_a = pc_k.clone();
        pc_k = pc_k[2..].clone();
        let var_name = pi32_a[0];
        i32_h = var_name;
        pi32_g = pc_k;
        // TODO
        // if (i32_h == i32_v) || (pi32_g = (i32_h + 1), pi32_g == 0x0) {
        //     return pi32_g;
        // }
        i32_h = -1;
        while {
            if i32_h == 0 {
                break;
            }

            i32_h = i32_h - 1;
            pi32_a = pc_k.clone();
            pc_k = pc_k[1..].clone();
            let val = pi32_a[0];
            val != '\0'
        } {}
    }
}

// WARNING: Removing unreachable block (ram,0x10002400)
// WARNING: Removing unreachable block (ram,0x10002422)

// WARNING: Removing unreachable block (ram,0x10002513)
// WARNING: Removing unreachable block (ram,0x10002557)

/*
Unable to decompile 'bad_1000_25f2'
// Cause:
Low-level Error: Overlapping input varnodes
*/

// WARNING (jumptable): Unable to track spacebase fully for stack
// WARNING (jumptable): Heritage AFTER dead removal. Example location: r0x10505fc2 : 0x1000270c

// WARNING: Restarted to delay deadcode elimination for space: ram

pub fn exported_stub_1000_29dc() -> u16 {
    if (___EXPORTEDSTUB != 0xb8) {
        return ctx.stack_seg_reg;
    }
    return uRam100029ed;
}

pub fn ___EXPORTEDSTUB() -> u16 {
    return 0;
}

// WARNING: Instruction at (ram,0x10083e27) overlaps instruction at (ram,0x10083e24)
//
// WARNING: Removing unreachable block (ram,0x10083e1f)

/*
Unable to decompile 'window_msg_func_1010_7300'
// Cause: Exception while decompiling 1010:7300: The pipe is being closed

*/

pub unsafe fn mixed_fn_1010_830a(param_1: u32, param_2: u16) -> u32 {
    let mut string_1: String;
    let mut local_bx_20: Struct449;
    let mut i_var2: i32;
    // let mut unaff_ss: u16;
    let in_struct_a: *mut Struct103;
    let mut u_var3: String;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut string_2: String;
    let mut local_8: u16;
    let mut local_6: u32;

    local_6 = 0;
    // TODO
    // local_bx_20 = (param_2 * 0x10); equiv to param_2 << 4
    // u_var3 = (param_1 >> 0x10);
    if local_bx_20.field_0x10 == 1 {
        // TODO
        //u_var1 = &local_bx_20.field_0x12;
        string_2 = set_error_mode_1010_8b14(ctx, param_1, &string_1);
        if (local_bx_20.field_0x12 == _local_a) && (local_bx_20.field_0x14 == (_local_a >> 0x10))
        {
            msg_box_1010_8bb4(ctx, param_1, &string_2);
            return 0;
        }
        let input_2 = CONCAT22(ctx.stack_seg_reg, local_2e);
        in_struct_a = process_struct_1008_48fe(ctx, input_2, 1, string_2);
        process_struct_1000_179c(0x1e, (in_struct_a >> 0x10));
        if in_struct_a == 0x0 {
            local_6 = 0;
        } else {
            local_6 = pass1_1008_3f92(in_struct_a, CONCAT22(unaff_ss, &local_2e));
        }
        close_file_1008_496c(ctx, &local_2e);
        _local_2e = local_6;
    } else {
        if ((param_2 * 0x10 + 0x10) == 2) {
            _local_2e = pass1_1010_878c(param_1, (param_2 * 0x10 + 0x16));
            if ((param_1 + 0x67c) == 0) {
                return 0;
            }
            i_var2 = param_2 * 0x10;
            pass1_1008_6562(
                (param_1 + 0x67c),
                CONCAT22((i_var2 + 0x1c), (i_var2 + 0x1e)),
                (i_var2 + 0x1a),
                _local_2e,
                (_local_2e >> 0x10),
            );
        } else {
            i_var2 = param_2 * 0x10;
            if ((i_var2 + 0x10) == 3) {
                string_1 = (i_var2 + 0x12);
                _local_2e = set_error_mode_1010_8b14(param_1, string_1, (string_1 >> 0x10));
                if (((i_var2 + 0x12) == _local_2e) && ((i_var2 + 0x14) == (_local_2e >> 0x10))) {
                    msg_box_1010_8bb4(param_1, _local_2e);
                    _local_2e = _local_2e;
                }
            } else {
                _local_2e = local_6;
                if ((param_2 * 0x10 + 0x10) == 4) {
                    string_1 = (param_2 * 0x10 + 0x12);
                    _local_2e = set_error_mode_1010_8b14(param_1, string_1, (string_1 >> 0x10));
                }
            }
        }
    }
    local_6 = _local_2e;
    return local_6;
}

/*
Unable to decompile 'win_fn_1018_6086'
// Cause:
Low-level Error: Symbol $$undef00000006 extends beyond the end of the address space
*/

// WARNI

/*
Unable to decompile 'load_accelerators_1020_41c8'
// Cause:
Low-level Error: Symbol $$undef0000000c extends beyond the end of the address space
*/

// WARNING: Instruction at (ram,0x10207c29) overlaps instruction at (ram,0x10207c28)
//

/*
Unable to decompile 'big_switch_statement_1020_c2f8'
// Cause: Exception while decompiling 1020:c2f8: The pipe is being closed

*/

// Unable to decompile 'bad_1028_e28a'
// WARNING: Instruction at (ram,0x10287af1) overlaps instruction at (ram,0x10287af0)
// WARNING: Instruction at (ram,0x10389f8e) overlaps instruction at (ram,0x10389f8d)
// WARNING: Instruction at (ram,0x1038aee3) overlaps instruction at (ram,0x1038aee2)
// WARNING: Instruction at (ram,0x1038cb02) overlaps instruction at (ram,0x1038cb00)
/*
Unable to decompile 'bad1_1038_de20'
// Cause: Exception while decompiling 1038:de20: The pipe is being closed
*/
// WARNING: Instruction at (ram,0x10407763) overlaps instruction at (ram,0x10407760)
// Unable to decompile 'pass1_1040_805a'
// WARNING: Instruction at (ram,0x1040d972) overlaps instruction at (ram,0x1040d96f)
// WARNING: Removing unreachable block (ram,0x1040d9cd)
// WARNING: Removing unreachable block (ram,0x1040d961)
// WARNING: Removing unreachable block (ram,0x1040d9c8)
// WARNING: Removing unreachable block (ram,0x1040d963)
// WARNING: Removing unreachable block (ram,0x1040d986)
// WARNING: Removing unreachable block (ram,0x1040d966)
// WARNING: Removing unreachable block (ram,0x1040d968)
// WARNING: Removing unreachable block (ram,0x1040d9d2)
// WARNING: Removing unreachable block (ram,0x1040d96d)
// WARNING: Removing unreachable block (ram,0x1040d9d4)
// WARNING: Removing unreachable block (ram,0x1040d97a)
// WARNING: Removing unreachable block (ram,0x1040d972)
// WARNING: Removing unreachable block (ram,0x1040d9c1)
// WARNING: Removing unreachable block (ram,0x1040d9bb)
// WARNING: Instruction at (ram,0x105024ee) overlaps instruction at (ram,0x105024ed)
// WARNING: Removing unreachable block (ram,0x105024ab)
// WARNING: Instruction at (ram,0x10505f2b) overlaps instruction at (ram,0x10505f2a)
// WARNING: Removing unreachable block (ram,0x10505a36)
// WARNING: Removing unreachable block (ram,0x1050578b)
// WARNING: Removing unreachable block (ram,0x1050571e)
// WARNING: Removing unreachable block (ram,0x105057b9)
// WARNING: Removing unreachable block (ram,0x1050582a)
// WARNING: Removing unreachable block (ram,0x10505a0b)
// WARNING: Removing unreachable block (ram,0x10505ce9)
// WARNING: Removing unreachable block (ram,0x10505ceb)
// WARNING: Removing unreachable block (ram,0x10505cf3)
// WARNING: Removing unreachable block (ram,0x10505d30)
// WARNING: Removing unreachable block (ram,0x10505d50)
// WARNING: Removing unreachable block (ram,0x10505d52)
// WARNING: Removing unreachable block (ram,0x10505d6b)
// WARNING: Removing unreachable block (ram,0x10505d54)
// WARNING: Removing unreachable block (ram,0x10505d6d)
// WARNING: Removing unreachable block (ram,0x10505d56)
// WARNING: Removing unreachable block (ram,0x10505d6f)
// WARNING: Removing unreachable block (ram,0x10505d58)
// WARNING: Removing unreachable block (ram,0x10505d5a)
// WARNING: Removing unreachable block (ram,0x10505d75)
// WARNING: Removing unreachable block (ram,0x10505d71)
// WARNING: Removing unreachable block (ram,0x10505d67)
// WARNING: Removing unreachable block (ram,0x10505d83)
// WARNING: Heritage AFTER dead removal. Example location: s0xfffe : 0x105056f7
// WARNING: Instruction at (ram,0x113800da) overlaps instruction at (ram,0x113800d9)
// WARNING: Instruction at (ram,0x11a004b9) overlaps instruction at (ram,0x11a004b7)
// WARNING: Removing unreachable block (ram,0x11a004a2)
// WARNING: Removing unreachable block (ram,0x11a00452)
// WARNING: Removing unreachable block (ram,0x11a00506)
// WARNING: Removing unreachable block (ram,0x11a00508)
// WARNING: Removing unreachable block (ram,0x11a0056f)
// WARNING: Removing unreachable block (ram,0x11a0050e)
// WARNING: Removing unreachable block (ram,0x11a00577)
// WARNING: Removing unreachable block (ram,0x11d80447)
// WARNING: Removing unreachable block (ram,0x11d80449)
// WARNING: Removing unreachable block (ram,0x11d804bb)
// WARNING: Removing unreachable block (ram,0x11d8043f)
// WARNING: Removing unreachable block (ram,0x11d80441)
// WARNING: Removing unreachable block (ram,0x11d804b0)

fn main() {
    let mut ctx: AppContext = AppContext::new();
    let mut param_1: String = String::new();
    let mut param_2: u16 = 0;
    let mut param_3: u16 = 0;
    let mut param_4: u16 = 0;
    let mut param_5: u16 = 0;
    let result = unsafe { entry(&mut ctx, &mut param_1, &mut param_2, param_3, param_4, param_5) };
}
