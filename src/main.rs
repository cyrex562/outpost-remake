extern crate libc;

use std::intrinsics::offset;

use defines::{AppContext, code};
use file_ops::close::close_file_1008_496c;
use func_ptr_funcs::{call_fn_ptr_1000_24cd, call_fn_ptr_1000_24db};
use mem_funcs::alloc_mem::alloc_mem_1000_167a;
use mem_funcs::mem_ops_1::{get_string_from_address, get_type_at_address};
use other_funcs::{big_fn_1010_b038, empty_fn_1000_55ac};
use pass::pass14_funcs::{pass1_1008_3f92, pass1_1008_57a4, pass1_1008_5b12, pass1_1008_6562, pass1_1008_6978};
use pass::pass8_funcs::pass1_1010_878c;
use pass::pass_funcs::{
    pass1_fn_1000_25a8, pass1_fn_1000_2913,
};
use sound_funcs::mci_fn_1020_08b6;
use string_ops::misc::process_string_1000_28dc;
use struct_ops::process_struct_1010_20ba;
use struct_ops::struct_ops_2::{process_struct_1000_179c, process_struct_1018_e91e};
use struct_ops::struct_ops_2::process_struct_1008_48fe;
use sys_ops::dos_ops::{dos3_call_1000_23ea, get_dos_env_1000_27d6};
use sys_ops::get_module_file_name_1000_262c;
use ui_ops::msg_box::msg_box_1010_8bb4;
use util::{CONCAT11, CONCAT22};
use winapi::{FatalAppExit16, FatalExit, GetVersion16, InitApp16, InitTask16, LockSegment16, make_htask, swi, WaitEvent16};

use crate::app_context::AppContext;
use crate::err_ops::set_error_mode_1010_8b14;
use crate::structs::prog_structs_24::Struct103;
use crate::structs::prog_structs_26::Struct183;
use crate::structs::prog_structs_2::Struct7;
use crate::structs::prog_structs_31::Struct449;

mod bad_funcs;
mod big_funcs;
mod bool_funcs;
mod cleanup;
mod draw;
mod err_ops;
mod exit;
mod file_ops;
mod func_ptr_funcs;
mod globals;
mod init;
mod list_funcs;
mod mem_funcs;
mod other_funcs;
mod sound_funcs;
mod ui_ops;
mod util;
mod loops;
mod constants;
mod structs;
mod app_context;
mod typedefs;
mod sys_structs;
mod winapi;
mod pass;
mod sys_ops;
mod struct_ops;
mod string_ops;

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
            let mut global_handle = LockSegment16(0xffff);
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
    call_fn_ptr_1000_24cd(ctx, Some(&fn_ptr_d));
    pass1_fn_1000_25a8(ctx);
    pass1_fn_1000_2913(ctx, 0);
    let in_str_1 = ctx.s_version__d__d_1050_0012[3..].clone();
    let mut string_e = process_string_1000_28dc(ctx, &in_str_1);
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

fn main() {
    let mut ctx: AppContext = AppContext::new();
    let mut param_1: String = String::new();
    let mut param_2: u16 = 0;
    let mut param_3: u16 = 0;
    let mut param_4: u16 = 0;
    let mut param_5: u16 = 0;
    let result = unsafe { entry(&mut ctx, &mut param_1, &mut param_2, param_3, param_4, param_5) };
}
